FROM gitpod/workspace-full

RUN bash -c ". /home/gitpod/.sdkman/bin/sdkman-init.sh && sdk install java 21.3.0.0-mandrel"