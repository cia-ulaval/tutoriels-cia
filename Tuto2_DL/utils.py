import torch
from torchmetrics.classification import MulticlassAccuracy
import lightning as L
import warnings

class AudioNet(torch.nn.Module):
    def __init__(self, n_classes=10, dropout_probability=0.2):
        super().__init__()
        self.n_classes = n_classes
        self.dropout_probability = dropout_probability
        self.cnn_layers = torch.nn.Sequential(
            torch.nn.Conv1d(in_channels=1, out_channels=32, kernel_size=8, stride=4),
            torch.nn.ReLU(),
            torch.nn.Dropout(self.dropout_probability),
            torch.nn.BatchNorm1d(32),
            torch.nn.Conv1d(in_channels=32, out_channels=64, kernel_size=8, stride=4),
            torch.nn.ReLU(),
            torch.nn.Dropout(self.dropout_probability),
            torch.nn.BatchNorm1d(64),
            torch.nn.Conv1d(in_channels=64, out_channels=128, kernel_size=8,stride=4),
            torch.nn.ReLU(),
            torch.nn.Dropout(self.dropout_probability),
            torch.nn.BatchNorm1d(128),
            torch.nn.MaxPool1d(5)
        )
        self.linear_layers = torch.nn.Sequential(
            torch.nn.Linear(128, 128),
            torch.nn.ReLU(),
            torch.nn.Dropout(self.dropout_probability),
            torch.nn.Linear(128, self.n_classes),
        )

    def forward(self, input):
        out = self.cnn_layers(input)
        return self.linear_layers(out.mean(-1))

class ClassificationModel(L.LightningModule):
    def __init__(self, model, optimizer="Adam", lr=1e-3):
        super().__init__()
        self.optimizer = optimizer
        self.lr = lr
        self.model = model
        self.loss = torch.nn.CrossEntropyLoss()
        self.metric = MulticlassAccuracy(num_classes=self.model.n_classes).to(self.device)
        self.conf_mat = torch.zeros(self.model.n_classes, self.model.n_classes)

    def training_step(self, batch, batch_idx):
        x, y = batch
        y_hat = self.model(x)
        train_acc = self.metric(torch.argmax(y_hat, dim=1), y)
        self.log("train_acc", train_acc, prog_bar=True)

        loss = self.loss(y_hat, y)
        self.log("train_loss", loss, prog_bar=True)
        return loss

    def predict_step(self, batch, batch_idx):
        x, y = batch
        y_hat = self.model(x)
        pred = y_hat.argmax(-1)
        for i in range(x.shape[0]):
          self.conf_mat[y.cpu()[i].item(), pred.cpu()[i].item()] += 1
        return None

    def validation_step(self, batch, batch_idx):
        x, y = batch
        y_hat = self.model(x)
        validation_acc = self.metric(torch.argmax(y_hat, dim=1), y)
        self.log("validation_acc", validation_acc)
        return torch.nn.CrossEntropyLoss(reduction='sum')(y_hat, y)


    def test_step(self, batch, batch_idx):
        x, y = batch
        y_hat = self.model(x)
        test_acc = self.metric(torch.argmax(y_hat, dim=1), y)
        self.log("test_acc", test_acc)

    def configure_optimizers(self):
      if self.optimizer == "Adam":
        return torch.optim.Adam(self.parameters(), lr=self.lr)
      elif self.optimizer == "SGD":
        return torch.optim.SGD(self.parameters(), lr=self.lr)
      

# @title Définition du jeu de données
# Inspiré de https://docs.deeplake.ai/4.1/guide/deep-learning/async-data-loader/
class AudioDataset(torch.utils.data.Dataset):
    def __init__(self, deeplake_ds):
        self.ds = deeplake_ds

    def __len__(self):
        return len(self.ds)

    def __getitem__(self, item):
        audio =  self.ds[item]["audio"].data(aslist=True)['value']
        target = self.ds[item]["labels"].data()['value']

        return audio, target


def collate_fn(data):
  """
  Avant de soumettre la batch de données au réseau de neurones, on peut vouloir
  traiter les données. La fonction collate_fn va traiter les données pour qu'elles
  puissent être utilisées correctement.

  Dans notre cas, puisque les données ne sont pas toutes de la même longueur, on
  va pad chaque batch pour que toutes les données soient de la même longueur.

  """
  audio = [torch.tensor(datapoint[0]).float() for datapoint in data]
  targets = torch.tensor([datapoint[1][0] for datapoint in data]).to(torch.int64)

  features = torch.nn.utils.rnn.pad_sequence(audio, batch_first=False).transpose(2,1).transpose(0,2)
  return features, targets

def compute_accuracy_and_conf_mat(model, dataloader, device):
    with warnings.catch_warnings():
        warnings.simplefilter("ignore") 
        model.eval()
        model.to(device)
        conf_mat = torch.zeros(10, 10)
        
        acc = 0
        with torch.no_grad():
            for _, sample in enumerate(dataloader):
            
                inputs, targets = sample
                
                inputs = inputs.to(device)
                targets = targets.to(device)
                
                outputs = model(inputs)
                pred = outputs.argmax(-1)
                acc += (targets == pred).sum().item()
                
                for i in range(inputs.shape[0]):
                  conf_mat[targets[i], pred[i]] += 1
        model.cpu()
    return acc, conf_mat