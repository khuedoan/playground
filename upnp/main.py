# Source: https://stackoverflow.com/a/35666064

import miniupnpc

upnp = miniupnpc.UPnP()

upnp.discoverdelay = 10
upnp.discover()

upnp.selectigd()

port = 43210

# addportmapping(external-port, protocol, internal-host, internal-port, description, remote-host)
upnp.addportmapping(port, 'TCP', upnp.lanaddr, port, 'testing', '')
