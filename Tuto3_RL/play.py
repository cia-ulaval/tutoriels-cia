from gymnasium.utils.play import play
import gymnasium as gym
import numpy as np
import argparse

def make_car():
    play(gym.make("CarRacing-v3", render_mode="rgb_array", max_episode_steps=-1),  
                keys_to_action={
                    "w": np.array([0, 0.7, 0], dtype=np.float32),
                    "a": np.array([-1, 0, 0], dtype=np.float32),
                    "s": np.array([0, 0, 1], dtype=np.float32),
                    "d": np.array([1, 0, 0], dtype=np.float32),
                    "wa": np.array([-1, 0.7, 0], dtype=np.float32),
                    "dw": np.array([1, 0.7, 0], dtype=np.float32),
                    "ds": np.array([1, 0, 1], dtype=np.float32),
                    "as": np.array([-1, 0, 1], dtype=np.float32),
                },
                noop=np.array([0, 0, 0.1], dtype=np.float32)
            )

def make_mountain():
    play(gym.make("MountainCar-v0", render_mode="rgb_array", max_episode_steps=-1),  
            keys_to_action={
                "a": 0,
                "d": 2,
            },
            noop=1
        )
    
def make_lunar():
    play(gym.make("LunarLander-v3", render_mode="rgb_array", max_episode_steps=-1, continuous=True),  
            keys_to_action={
                "a": np.array([0, 0.7], dtype=np.float32),
                "s": np.array([0.7, 0], dtype=np.float32),
                "d": np.array([0, -0.7], dtype=np.float32),
                "ds": np.array([0.7,-0.7], dtype=np.float32),
                "as": np.array([0.7, 0.7], dtype=np.float32),
            },
            noop=np.array([0, 0, 0.1], dtype=np.float32)
        )
    

if __name__ == "__main__":
    parser = argparse.ArgumentParser(
                    prog='ProgramName',
                    description='What the program does',
                    epilog='Text at the bottom of help')
    parser.add_argument("--game", type=str, default="Lunar")
    args = parser.parse_args()

    if args.game == "Car":
        make_car()
    elif args.game == "Mountain":
        make_mountain()
    elif args.game == "Lunar":
        make_lunar()
    else:
        make_car()