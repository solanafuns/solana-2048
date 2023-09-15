import * as borsh from "borsh";

class Game {
  steps: number;
  ended: boolean;
  max: number;
  board: number[][];

  constructor() {
    this.steps = 0;
    this.ended = false;
    this.max = 0;
    this.board = [[]];
  }

  toBytes() {
    return borsh.serialize(schema, this);
  }
}

const schema = {
  struct: {
    steps: "u16",
    ended: "bool",
    max: "u16",
    board: {
      array: {
        type: {
          array: {
            type: "u16",
          },
        },
      },
    },
  },
};

const ParseGame = (data: Buffer) => {
  return borsh.deserialize(schema, data);
};

const MATH_STUFF_SIZE = borsh.serialize(schema, new Game()).length;

export { Game, MATH_STUFF_SIZE, ParseGame };
