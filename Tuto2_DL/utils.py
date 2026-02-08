import torch
import warnings
from sklearn.datasets import load_breast_cancer

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
    
class BreastCancerDataset(torch.utils.data.Dataset):
    def __init__(self):
        data = load_breast_cancer()
        X = data.data             # shape: (569, 30)
        y = data.target           # shape: (569,)

        self.X = torch.tensor(X, dtype=torch.float32)
        self.y = torch.tensor(y, dtype=torch.long)  # classification: 0 or 1

    def __len__(self):
        return len(self.X)

    def __getitem__(self, idx):
        x = self.X[idx]
        y = self.y[idx]
        return x, y


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

def compute_accuracy_and_conf_mat(model, dataloader, device, n_classes=2):
    with warnings.catch_warnings():
        warnings.simplefilter("ignore") 
        model.eval()
        model.to(device)
        conf_mat = torch.zeros(n_classes, n_classes)
        
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