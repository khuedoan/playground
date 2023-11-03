#!/bin/sh

# Setup NGINX reverse proxy.
sudo dnf install -y nginx
sudo systemctl enable --now nginx

# Copy a basic HTML page.
sudo cp ./index.html /usr/share/nginx/html/

# Setup firewall to allow HTTP access (port 80).
sudo firewall-cmd --add-port=80/tcp
sudo firewall-cmd --reload --permanent

# Expose to the internet using a Tunnel service.
# Not everyone can port-forward :)
sudo dnf config-manager --add-repo https://pkgs.tailscale.com/stable/fedora/tailscale.repo
sudo dnf install -y tailscale
sudo systemctl enable --now tailscaled
sudo tailscale up
sudo tailscale funnel --bg 80
