# The Inverted Pendulum
Welcome! The purpose of this project is to implement a state-space closed feedback controller that will stabilize a pendulum in the upright vertical position.
If this is your first time hearing about the inverted pendulum problem, feel free to see the following wiki page for more context.
- https://en.wikipedia.org/wiki/Inverted_pendulum

## Getting started
A local jupyter environment for creating plots and simulations is already provided. This environment comes prepackaged 
with common/popular libraries that scientists have come to expect when working with python. Specifics of what is contained
within the environment can be found [here](https://jupyter-docker-stacks.readthedocs.io/en/latest/using/selecting.html#jupyter-scipy-notebook).

All you need to do is install tools that give you access to the environment.

### Requirements

 - A linux development environment
 - [Docker](https://docs.docker.com/engine/install/)
 - [docker-compose](https://docs.docker.com/compose/install/)
 - [VSCode](https://code.visualstudio.com/)
 - VSCode Extention: [Remote - Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)

The remaining instructions will assume you have these requirements correctly installed.

### Setup
1. Open VSCode.

2. Open the repository within a remote container by selecting the green `><` icon in the bottom left corner and select `Remote-Containers: Reopen in Container`. ![remote container](images/open_remote_container.png)

3. Select the directory containing the `docker-compose.yml` of this repository. VSCode should reopen and begin building the docker image for the environment. This can take up to 5 minutes the first time.

4. VSCode should now be loaded with the development environment. To test, open the `simulation/placeholder.ipynb` file
and try evaluating some of the  cells and making sure you can produce outputs.

Congratulations. You should now have a fully functional environment.
