#ifndef QUEUE_H   /* Include guard */
#define QUEUE_H

typedef struct Coord {
    int row;
    int col;
} Coord;


typedef struct Node {
    struct Coord location;
    int distance;
    struct Node* next;
} Node;


typedef struct Queue {
    struct Node *front, *rear;
} Queue;


struct Node* newNode(Coord, int);
struct Queue* createQueue();
void enqueue(struct Queue*, Coord, int);
void dequeue(struct Queue*);

#endif // QUEUE_H_

