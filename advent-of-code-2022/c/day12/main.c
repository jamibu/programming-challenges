#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "queue.h"


#define ROWS 41
#define COLS 183
#define filename "puzzleInput.txt"

// #define ROWS 5
// #define COLS 10
// #define filename "example.txt"


// Part 2. Start at End and BFS until you hit any a

int readMaze(size_t cols, char maze[][cols], char fname[]) {
    FILE *fp = NULL;
    int i = 0;

    fp = fopen(fname, "r");

    while(fgets(maze[i], cols, fp)) {
        // Don't want \n in string
        maze[i][strlen(maze[i]) - 1] = '\0';
        i++;
    }

    fclose(fp);

    return 1;
}


int withinMaze(Coord* location, size_t rows, size_t cols) {
    return (location->row >= 0) && (location->row < rows) &&
           (location->col >= 0) && (location->col < cols);
}


int traversable(Coord* location1, Coord* location2, size_t cols, char maze[][cols]) {
    int elevation1 = maze[location1->row][location1->col];
    int elevation2 = maze[location2->row][location2->col];
    return (elevation2 - elevation1 <= 1);
}


void showVisited(size_t rows, size_t cols, int visited[][cols], char maze[][cols]) {
    for (int i = 0; i < rows; i++) {
        for (int j = 0; j < cols; j++) {
            if (visited[i][j] == 1) {
                printf("%c", ' ');
            } else {
                printf("%c", maze[i][j]);
            }
        }
        printf("\n");
    }
}


int bfs(size_t rows, size_t cols, char maze[][cols], Coord start, Coord end, int part) {
    int visited[rows][cols];
    memset(visited, 0, sizeof(visited));

    visited[start.row][start.col] = 1;

    Queue* queue = createQueue();
    // Start has distance of 0
    enqueue(queue, start, 0);

    int rowNum[4] = {-1, 0, 0, 1};
    int colNum[4] = {0, -1, 1, 0};


    while (queue->front != NULL) {
        Coord location = queue->front->location;
        int distance = queue->front->distance;


        if (part == 1) {
            if (location.row == end.row && location.col == end.col) {
                return distance;
            }
        }

        if (part == 2) {
            if (maze[location.row][location.col] == 'a') {
                return distance;
            }
        }

        dequeue(queue);

        for (int i = 0; i < 4; i++) {
            Coord newLocation;
            newLocation.row = location.row + rowNum[i];
            newLocation.col = location.col + colNum[i];

            int w, t, v;
            w = withinMaze(&newLocation, rows, cols);
            v = visited[newLocation.row][newLocation.col] == 0;
            if (part == 1) {
                t = traversable(&location, &newLocation, cols, maze);
            }

            if (part == 2) {
                t = traversable(&newLocation, &location, cols, maze);
            }

            if (w && t && v) {
                visited[newLocation.row][newLocation.col] = 1;
                enqueue(queue, newLocation, distance+1);
            }
        }
    }

    showVisited(rows, cols, visited, maze);

    return 0;
}


int part1(size_t rows, size_t cols, char maze[][cols], Coord start, Coord end) {

    printf("Starting at: row=%d, col=%d\n", start.row, start.col);
    printf("Ending at:   row=%d, col=%d\n", end.row, end.col);

    int dist = bfs(ROWS, COLS, maze, start, end, 1);
    printf("Part One: %d\n", dist);

    return dist;
}


int part2(size_t rows, size_t cols, char maze[][cols], Coord peak) {
    for (int i = 0; i < ROWS; i++) {
        for (int j = 0; j < COLS; j++) {
            if (maze[i][j] == 'E') {
                peak.row = i;
                peak.col = j;
                maze[i][j] = 'a';
            }
        }
    }

    printf("Starting at: row=%d, col=%d\n", peak.row, peak.col);

    int dist = bfs(ROWS, COLS, maze, peak, peak, 2);
    printf("Part One: %d\n", dist);

    return dist;
}


int main(void) {
    char maze[ROWS][COLS];

    readMaze(COLS, maze, filename);

    Coord start;
    Coord end;

    for (int i = 0; i < ROWS; i++) {
        for (int j = 0; j < COLS; j++) {
            if (maze[i][j] == 'S') {
                start.row = i;
                start.col = j;
                maze[i][j] = 'a';
            } else if (maze[i][j] == 'E') {
                end.row = i;
                end.col = j;
                maze[i][j] = 'z';
            }
        }
    }

    part1(ROWS, COLS, maze, start, end);
    part2(ROWS, COLS, maze, end);

    return 1;
}

