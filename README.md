# Stock math api

Search for the best way through the warehouse to collect different goods for further shipment to the client.

## API Documentation

#### POST /calculate_optimal_path

request body:

- `geometry` stock geometry how 2d array when i index how Y and j index how X
- `points` is array of points (i,j) on which a person stands when he takes goods from the shelves.

responce body:

- `algorithm` is algorithm type
- `distance_matrix_time` how many time need for calculate graph (more details in "Solution" section)
- `traveling_salesman_time` how many time need for calculate min path from graph (more details in "Solution" section)
- `distance` how many steps need for getting goods from stock all shelves
- `path` route min path

#### Example

```
POST /calculate_optimal_path
body
{
  "geometry": [
    ["W","W","W","W","W","W","W","W"],
    ["W","P","_","_","_","P","_","W"],
    ["W","W","W","_","W","W","W","W"],
    ["W","_","_","_","_","_","P","W"],
    ["W","W","W","_","W","W","W","W"],
    ["W","P","_","_","_","_","_","W"],
    ["W","W","W","_","W","W","W","W"],
    ["W","_","_","_","_","_","P","W"],
    ["W","S","_","_","_","_","_","_"],
    ["W","W","W","W","W","W","W","W"]
  ],
  "points": [
    [8, 1],
    [1, 1],
    [1, 5],
    [3, 6],
    [5, 1],
    [7, 6]
  ]
}
return:
{
    "algorithm": "simulated_annealing",
    "distance_matrix_time": "162.47µs",
    "traveling_salesman_time": "1.000004241s",
    "distance": 48.0,
    "path": [
        [8, 1],
        [1, 5],
        [1, 1],
        [3, 6],
        [5, 1],
        [7, 6],
        [8, 1]
    ]
}
```

## Development

To start server use command: `cargo run --bin chat`

Open url: [http://localhost:8080/](http://localhost:8080/)

# Docker

## Build image

```bash
$ docker build --target runtime -t stock_math_api .
```

## Run tests

```bash
$ docker build --progress=plain --target test -t chat-test .
```

## Postmen examples

stock_math_api.postman_collection.json - located in repo

## Solution

1.  Calculate graph when set of nodes is 'Pi' and set of edges `('Pi', 'Pj') = min_path(geometry, Pi, Pj)` min_path we calculate use bfs algorithm.
2.  Solve traveling salesman problem for all nodes in calculated graph using [Simulated_annealing](https://en.wikipedia.org/wiki/Simulated_annealing) algorithm.

## Geometry example

W - wall or shelf
P - point
S - start point

```
WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW
WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW
WP_____P______P_______P________P________P______W
WWWWWWWWWWWWWWWWWWWWWWWWW__WWWWWWWWWWWWWWWWWWWWW
WWWWWWWWWWWWWWWWWWWWWWWWW__WWWWWWWWWWWWWWWWWWWWW
W________________P__P______________P________P__W
WWWWWWWWWWWWWWWWWWWWWWWWW__WWWWWWWWWWWWWWWWWWWWW
WWWWWWWWWWWWWWWWWWWWWWWWW__WWWWWWWWWWWWWWWWWWWWW
W__P_________P________________P________P_______W
WWWWWWWWWWWWWWWWWWWWWWWWW__WWWWWWWWWWWWWWWWWWWWW
WWWWWWWWWWWWWWWWWWWWWWWWW__WWWWWWWWWWWWWWWWWWWWW
WP_______________P__________________P_________PW
WWWWWWWWWWWWWWWWWWWWWWWWW__WWWWWWWWWWWWWWWWWWWWW
WWWWWWWWWWWWWWWWWWWWWWWWW__WWWWWWWWWWWWWWWWWWWWW
W___P___________P______________P__________P____W
WWWWWWWWWWWWWWWWWWWWWWWWW__WWWWWWWWWWWWWWWWWWWWW
WWWWWWWWWWWWWWWWWWWWWWWWW__WWWWWWWWWWWWWWWWWWWWW
W____________P________P_____________P________P_W
WWWWWWWWWWWWWWWWWWWWWWWWW__WWWWWWWWWWWWWWWWWWWWW
WWWWWWWWWWWWWWWWWWWWWWWWW__WWWWWWWWWWWWWWWWWWWWW
W______________________________________________W
WWWWWWWWWWWWWWWWWWWWWWWWW__WWWWWWWWWWWWWWWWWWWWW
WWWWWWWWWWWWWWWWWWWWWWWWW__WWWWWWWWWWWWWWWWWWWWW
W__P___________P____________________PP_________W
WWWWWWWWWWWWWWWWWWWWWWWWW__WWWWWWWWWWWWWWWWWWWWW
WWWWWWWWWWWWWWWWWWWWWWWWW__WWWWWWWWWWWWWWWWWWWWW
W__________________P_______P__________________PW
WWWWWWWWWWWWWWWWWWWWWWWWW__WWWWWWWWWWWWWWWWWWWWW
WWWWWWWWWWWWWWWWWWWWWWWWW__WWWWWWWWWWWWWWWWWWWWW
W_____P_____________________P_________P________W
WWWWWWWWWWWWWWWWWWWWWWWWW__WWWWWWWWWWWWWWWWWWWWW
WWWWWWWWWWWWWWWWWWWWWWWWW__WWWWWWWWWWWWWWWWWWWWW
W______________________________________________W
W______________________________________________W
W_______________________________________________
W___S___________________________________________
WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW

```
