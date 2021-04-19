# Cloudflare Tunnel

<https://blog.cloudflare.com/tunnel-for-everyone/>
<https://developers.cloudflare.com/cloudflare-one/tutorials/single-command>
<https://developers.cloudflare.com/cloudflare-one/tutorials/share-new-site>

```
cloudflared login
kubectl create secret generic cloudflared --from-file=certificate=$HOME/.cloudflared/cert.pem
```
