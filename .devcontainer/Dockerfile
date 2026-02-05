FROM  archlinux:latest

# Install dependencies
RUN	 pacman -Syu --noconfirm sudo openssh bash-completion \
    vim git zig helix podman \
    base-devel \
    rust rust-src rust-analyzer cargo

# Generate host keys
RUN  /usr/bin/ssh-keygen -A

# Add defualt user
RUN  useradd b3nt
RUN  mkdir -p /home/b3nt/.ssh
RUN  chmod -R 700 /home/b3nt/.ssh/
RUN  chown -R b3nt:b3nt /home/vscode/
RUN  usermod -aG wheel b3nt
RUN  echo '%wheel ALL=(ALL:ALL) NOPASSWD: ALL' >> /etc/sudoers
RUN	 echo 'b3nt:1234' | chpasswd

# Add password to root user
RUN	 echo 'root:1234' | chpasswd

# Config sshd
RUN  sed -i -e 's/^#PasswordAuthentication yes/PasswordAuthentication no/g' /etc/ssh/sshd_config
RUN  sed -i -e 's/^#PubkeyAuthentication yes/PubkeyAuthentication yes/g' /etc/ssh/sshd_config
RUN  sed -i -e 's/^UsePAM yes/UsePAM no/g' /etc/ssh/sshd_config

# Run openssh daemon
CMD  ["/usr/sbin/sshd", "-D"]

# Set default user
USER  vscode
