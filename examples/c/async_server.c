#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>
#include <sys/epoll.h>
#include <fcntl.h>
#include <errno.h>

#define PORT 8080
#define BUFFER_SIZE 1024
#define MAX_EVENTS 10

int set_nonblocking(int fd) {
    int flags = fcntl(fd, F_GETFL, 0);
    if (flags == -1) {
        perror("fcntl");
        return -1;
    }
    if (fcntl(fd, F_SETFL, flags | O_NONBLOCK) == -1) {
        perror("fcntl");
        return -1;
    }
    return 0;
}

void handle_client(int client_socket) {
    char buffer[BUFFER_SIZE];
    int bytes_read = recv(client_socket, buffer, BUFFER_SIZE - 1, 0);

    if (bytes_read <= 0) {
        if (bytes_read == 0 || (bytes_read == -1 && errno != EAGAIN)) {
            // Connection closed or error
            close(client_socket);
        }
        return;
    }

    buffer[bytes_read] = '\0';
    printf("Received request:\n%s\n", buffer);

    const char *response =
        "HTTP/1.1 200 OK\r\n"
        "Content-Type: text/html\r\n"
        "Content-Length: 13\r\n"
        "\r\n"
        "Hello, world!";

    send(client_socket, response, strlen(response), 0);
}

int main() {
    int server_socket, client_socket, epoll_fd, n;
    struct sockaddr_in server_addr, client_addr;
    socklen_t addr_len = sizeof(client_addr);
    struct epoll_event ev, events[MAX_EVENTS];

    // Create server socket
    server_socket = socket(AF_INET, SOCK_STREAM, 0);
    if (server_socket == -1) {
        perror("socket");
        exit(EXIT_FAILURE);
    }

    // Set server socket to non-blocking
    if (set_nonblocking(server_socket) == -1) {
        close(server_socket);
        exit(EXIT_FAILURE);
    }

    // Bind socket to address
    server_addr.sin_family = AF_INET;
    server_addr.sin_addr.s_addr = INADDR_ANY;
    server_addr.sin_port = htons(PORT);

    if (bind(server_socket, (struct sockaddr *)&server_addr, sizeof(server_addr)) == -1) {
        perror("bind");
        close(server_socket);
        exit(EXIT_FAILURE);
    }

    // Listen for connections
    if (listen(server_socket, 10) == -1) {
        perror("listen");
        close(server_socket);
        exit(EXIT_FAILURE);
    }

    // Create epoll instance
    epoll_fd = epoll_create1(0);
    if (epoll_fd == -1) {
        perror("epoll_create1");
        close(server_socket);
        exit(EXIT_FAILURE);
    }

    // Add server socket to epoll
    ev.events = EPOLLIN;
    ev.data.fd = server_socket;
    if (epoll_ctl(epoll_fd, EPOLL_CTL_ADD, server_socket, &ev) == -1) {
        perror("epoll_ctl");
        close(server_socket);
        close(epoll_fd);
        exit(EXIT_FAILURE);
    }

    printf("Server listening on port %d...\n", PORT);

    // Event loop
    while (1) {
        int n_fds = epoll_wait(epoll_fd, events, MAX_EVENTS, -1);
        if (n_fds == -1) {
            perror("epoll_wait");
            break;
        }

        for (n = 0; n < n_fds; ++n) {
            if (events[n].data.fd == server_socket) {
                // Accept new connection
                client_socket = accept(server_socket, (struct sockaddr *)&client_addr, &addr_len);
                if (client_socket == -1) {
                    perror("accept");
                    continue;
                }

                // Set client socket to non-blocking
                if (set_nonblocking(client_socket) == -1) {
                    close(client_socket);
                    continue;
                }

                // Add client socket to epoll
                ev.events = EPOLLIN | EPOLLET;
                ev.data.fd = client_socket;
                if (epoll_ctl(epoll_fd, EPOLL_CTL_ADD, client_socket, &ev) == -1) {
                    perror("epoll_ctl");
                    close(client_socket);
                    continue;
                }
            } else {
                // Handle client request
                handle_client(events[n].data.fd);
            }
        }
    }

    // Clean up
    close(server_socket);
    close(epoll_fd);
    return 0;
}

