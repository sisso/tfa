# Dummy Key provider

## Running

    # server start
    $ tfa --port 8881

    # one device request a key
    $ curl localhost:8881/keys/123
    {"value": null}

    # admin provide the value
    curl localhost:8881/kys/1243 -X POST -d "ha" -v

    # the device retry
    $ curl localhost:8881/keys/123
    {"value": "ha"}
    
    # any subsequently request after the first value return is back to null 
    $ curl localhost:8881/keys/123
    {"value": null}
    