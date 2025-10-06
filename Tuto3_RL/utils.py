import torch

def get_device():
    if torch.cuda.is_available():
        return torch.device('cuda')
    elif torch.backends.mps.is_available():
        return torch.device('mps')
    else:
        return torch.device("cpu")

def verify_shape(V, b_rtgs):
    if len(V.shape) > len(b_rtgs.shape):
        return V, b_rtgs.unsqueeze(1)
    elif len(V.shape) < len(b_rtgs.shape):
        return V.unsqueeze(0), b_rtgs
    else:
        return V, b_rtgs