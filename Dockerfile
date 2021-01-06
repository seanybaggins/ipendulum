FROM jupyter/scipy-notebook

# Switching permissions to root to install global packages
USER root
RUN apt-get update && apt-get install gcc

USER jovyan
RUN pip3 install control
RUN conda install -c conda-forge slycot