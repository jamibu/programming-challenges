#include <stdlib.h>
#include "queue.h"


Node* newNode(Coord location, int distance) {
    Node* node = (Node*)malloc(sizeof(Node));
    node->location = location;
    node->distance = distance;
    node->next = NULL;
    return node;
}


Queue* createQueue() {
    Queue* queue = (Queue*)malloc(sizeof(Queue));
    queue->front = NULL;
    queue->rear = NULL;

    return queue;
}


void enqueue(Queue* queue, Coord location, int distance) {
    Node* node = newNode(location, distance);

    if (queue->rear == NULL) {
        queue->front = queue->rear = node;
        return;
    }

    queue->rear->next = node;
    queue->rear = node;
}


void dequeue(Queue* queue) {
    if (queue->front == NULL)
        return;

    Node* temp = queue->front;
    queue->front = queue->front->next;

    if (queue->front == NULL)
        queue->rear = NULL;

    free(temp);
}

