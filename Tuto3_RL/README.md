# README

Contrairement au dernier laboratoire, vous aurez besoin d'une version de Python fonctionnelle ainsi que d'un environnement virtuel. Je vous propose donc d'installer ``uv'', un outil qui vous permettra d'installer Python, de créer votre environnement virtuel ainsi que d'installer toutes les librairies nécessaires pour faire fonctionner ce notebook. 

Je vous invite donc à ouvrir un terminal et à rouler les commandes suivantes. La dernière commande devrait ouvrir une fenêtre avec Jupyter Notebook. Ouvrez le fichier `tutoriel_h2025_CIA_réseaux_de_neurones.ipynb`.

### Pour MacOs et Linux:

1. curl -LsSf https://astral.sh/uv/install.sh | sh
2. uv python install 3.12.3 
3. uv venv tutoCiaEnv --python 3.12.3
4. source tutoCiaEnv/bin/activate
5. uv pip install notebook
6. jupyter notebook

Pour Mac, la commande `brew install uv` devrait fonctionner aussi, au lieu d'utiliser `curl`. 

### Pour Windows
1. powershell -ExecutionPolicy ByPass -c "irm https://astral.sh/uv/install.ps1 | iex"
2. uv python install 3.12.3 
3. uv venv tutoCiaEnv --python 3.12.3
4. source tutoCiaEnv/bin/activate
5. uv pip install notebook
6. jupyter notebook