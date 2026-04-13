#pragma once

#include <stdio.h>

#include "config.h"

/* HTTP server configuration. */
struct Server {
    int port;
    const char* host;
};

void server_start(struct Server srv) {
    printf("Starting %s:%d with config %p\\n", srv.host, srv.port, Config());
}
