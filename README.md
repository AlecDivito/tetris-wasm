## Tetris clone
Just another tetris clone thats going to be a little over board.
Built with HTML, CSS, vanilla javascript and rust right now, still
deciding what framework I may want to use.

## Project Status
This project is still a work in progress but the hope is to get it to
the same point as my [Minesweeper / Proxx Clone (Bomb Finder)](https://github.com/AlecDivito/bomb-finder)
project is at. So a PWA that is installable to a users phone or computer
with use of indexDB to store intresting statistics of the games played.

## Project Screen Shots
![Screen shot of current build of tetris project](https://raw.githubusercontent.com/AlecDivito/web-resume/master/src/data/images/tetris.png)

## Installation and Setup Instructions

Clone down this repository. You will need `node` and `npm` installed
globally on your machine as `rust` and `wasm-bindgen`.

Web Installation:
`
cd web-tetris-frontend
npm install
`

Tetris Logic Installation:
`
cd tetris-logic
wasm-bindgen build
`

To Start Server: *Note must build both projects before running*
`npm run start`

To Visit App:
`localhost:9966`

## Reflections

This was my first project using rust and webassembly. It is also my first
project working with the canvas element (which is really fun btw). This
project was used to initially learn about the 2 technologies.

Tetris was a fun game growing up so I wanted my own verson of it. Right now
it's really simple (about 90% of the commpleted core game) but I want to extend
it to use more of the web browsers features and API's. I thought making a game
would be a great way to learn about rust, webassembly and browsers perforamce
tools.

The main challenge I faced during the first stage of development of this project
was implmenting the acutally game logic. The rotation of the pieces was difficult
to figure out at first and the shadow piece was also hard to implement (it's
actually not 100% working right now anyway). There is still a lot to do so I'm
putting off thinking about major features that can be added.

At the end of the day this project is ment to be a completed project that shows
off some of my skills in diffrent settings using a varity of diffrent tools. It's
learning to roll my own webpack build process and creating my own UI. It's also
being used to hone my rust skill set and get more comfotable with webassembly.

## Whats left

The majority of the tetris game is completed and works however there are some
key options that i think would be fun to make avaliable to the game:

1. Have the option to have a [Lock delay](https://tetris.fandom.com/wiki/Lock_delay)
2. Allow for different type of [Top out's](https://tetris.fandom.com/wiki/Top_out)
3. Provide access to the game to check when a row is completed and which rows so animations can play
4. Have the option to configure the randomizer like he so called 7-bag random generator. I also saw the fun idea of letting a user provide their own randomizer.
5. Implement SRS rotation and assocaited scroing as seen below.

| Action                                     | Point Value |
| -------------------------------------------|-------------|
| Mini T-Spin                                | 100×level   |
| Mini T-Spin Single                         | 200×level   |
| T-Spin/Mini T-Spin Double                  | 400×level   |
| B2B Mini T-Spin Double                     | 600×level   |
| T-Spin Single	                             | 800×level   |
| B2B T-Spin Single/B2B Tetris/T-Spin Double | 1,200×level |
| T-Spin Triple                              | 1,600×level |
| B2B T-Spin Double	                         | 1,800×level |
| B2B T-Spin Triple	                         | 2,400×level |


## Resources
Tetris Game Rules:
- https://tetris.fandom.com/wiki/Tetris_Guideline

Also some stuff that looks intresting and helpful but haven't read yet:
- https://liacs.leidenuniv.nl/~kosterswa/tetris/tot.pdf
- https://tetris.wiki/Tetris_Guideline

## Build docker file

```bash
docker buildx build \
    --platform "linux/amd64" \
    --provenance false \
    --file "Dockerfile" \
    --tag "registry/alecdivito/tetris:v1.0.0" \
    --push \
    "."
```