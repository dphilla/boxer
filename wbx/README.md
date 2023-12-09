# Recognized Dockerfile Directives

1. **FROM**
   - Initializes a new build stage and sets the base layer.

2. **RUN**
   - Executes commands in a new layer on top of the current layer and commits the results.
   - Forms:
     - Shell form: `RUN <command>`
     - Exec form: `RUN ["executable", "param1", "param2"]`

3. **CMD**
   - Provides defaults for an executing container. Only one CMD instruction in a Dockerfile.
   - Forms:
     - Shell form: `CMD <command>`
     - Exec form: `CMD ["executable", "param1", "param2"]`

4. **LABEL**
   - Adds metadata to a layer as key-value pairs.
   - Syntax: `LABEL <key>=<value> ...`

5. **EXPOSE**
   - Informs Docker that the container listens on specified network ports at runtime.
   - Syntax: `EXPOSE <port> [<port>/<protocol>...]`

6. **ENV**
   - Sets environment variables.
   - Syntax: `ENV <key>=<value> ...`

7. **ADD**
   - Copies new files, directories, or remote file URLs and adds them to the layer's filesystem.
   - Syntax: `ADD <src>... <dest>`

8. **COPY**
   - Copies new files or directories and adds them to the filesystem of the container.
   - Syntax: `COPY <src>... <dest>`
   - Flags:
     - `--chown=<user>:<group>`: Set the user and group ownership of the copied files.

9. **ENTRYPOINT**
   - Configures a container that will run as an executable.
   - Forms:
     - Shell form: `ENTRYPOINT <command>`
     - Exec form: `ENTRYPOINT ["executable", "param1", "param2"]`

10. **VOLUME**
    - Creates a mount point with the specified name.
    - Syntax: `VOLUME ["<path>"]`

11. **USER**
    - Sets the username or UID for running the layer and following `RUN`, `CMD`, and `ENTRYPOINT` instructions.
    - Syntax: `USER <user>[:<group>]`

12. **WORKDIR**
    - Sets the working directory for `RUN`, `CMD`, `ENTRYPOINT`, `COPY`, and `ADD` instructions.
    - Syntax: `WORKDIR <path>`

13. **ARG**
    - Defines a variable that users can pass at build-time to the builder.
    - Syntax: `ARG <name>[=<default value>]`

14. **ONBUILD**
    - Adds a trigger instruction to be executed when the layer is used as the base for another build.
    - Syntax: `ONBUILD <INSTRUCTION>`

15. **STOPSIGNAL**
    - Sets the system call signal that will be sent to the container to exit.
    - Syntax: `STOPSIGNAL <signal>`

16. **HEALTHCHECK**
    - Tells Docker how to test a container to check that it is still working.
    - Syntax: `HEALTHCHECK [OPTIONS] CMD <command>`
    - Options:
      - `--interval=<interval>`: Time between running the check.
      - `--timeout=<timeout>`: Time to wait before considering the check to have hung.
      - `--start-period=<start-period>`: Time to wait before starting checks.
      - `--retries=<retries>`: Consecutive failures needed to consider a container as unhealthy.

17. **SHELL**
    - Allows the default shell used for the shell form of commands to be overridden.
    - Syntax: `SHELL ["executable", "parameters"]`

18. **MAINTAINER** (deprecated)
    - Used to set the author field of the generated layer.
    - Syntax: `MAINTAINER <name>`

