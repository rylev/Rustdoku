async function setUp() {
  await wasm_bindgen('./pkg/rustdoku_js_bg.wasm');
}

function generateWasm(n) {
  const { Game } = wasm_bindgen;
  return new Game(n)
}

Vue.component('cell', {
  props: ['data'],
  template: `
    <div v-if="editing" class="cell"><input class="input" type="number" @blur="submit" @keydown="keydown"></div>
    <div v-else="editing" class="cell" @click="editMode">{{data}}</div>
  `,
  data() {
    return {
      editing: false
    }
  },
  methods: {
    editMode() {
      if (!this.data) {
        this.editing = true
      }
    },
    submit(e) {
      console.log(e)
    },
    keydown(e) {
      let value = e.target.value
      if ((value.length >= 1 && e.key !== "Backspace") || e.key === "0") {
        e.preventDefault()
      }
    },
  }
})

Vue.component('cellRow', {
  props: ['data'],
  template: `
    <div class="cellRow" style="display: flex">
      <cell :data="data[0]"></cell>
      <cell :data="data[1]"></cell>
      <cell :data="data[2]"></cell>
    </div>
  `
})

Vue.component('square', {
  props: ['data'],
  template: `
    <div class="square" style="display: flex; flex-direction: column;">
      <cellRow :data="data.slice(0,3)"> </cellRow>
      <cellRow :data="data.slice(3,6)"> </cellRow>
      <cellRow :data="data.slice(6,9)"> </cellRow>
    </div>
  `,
})

Vue.component('row', {
  props: ['data'],
  template: `
    <div class="row" style="display: flex">
      <square :data="data[0]"></square>
      <square :data="data[1]"></square>
      <square :data="data[2]"></square>
    </div>
  `,
})

Vue.component('board', {
  props: ['board'],
  template: `
    <div class="board" style="display: flex; flex-direction: column" >
      <row :data="squareRows[0]"></row>
      <row :data="squareRows[1]"></row>
      <row :data="squareRows[2]"></row>
    </div>
  `,
  computed: {
    squareRows() {
      const squares = this.board.squares()
      const rows = []
      for ([i, square] of squares.entries()) {
        if (rows[i % 3] === undefined) {
          rows[i % 3] = []
        }
        rows[i % 3].push(square)
      }
      return rows
    }
  }
})

Vue.component('playarea', {
  props: ['initialBoard'],
  template: `
    <div class="playarea">
      <board :board="board"></board>
      <div class="button solve" @click="solve">Solve</div>
      <div class="button new" @click="generateNewWasm">Generate New With Wasm</div>
      <div class="button new" @click="generateNewJS">Generate New With Wasm</div>
    </div>
  `,
  methods: {
    solve() {
      const result = this.board.solve()
      Vue.set(this, 'board', result.game());
    },
    generateNewJs() {
      const totalBoards = 5
      const totalRuns = 5

      let best = undefined
      let mostSteps = 0
      for (let numberOfBlanks = 45; numberOfBlanks < 50; numberOfBlanks++) {
        for (let boardCount = 0; boardCount < totalBoards; boardCount++) {
          const newBoard = generateJs(numberOfBlanks)
          let total = 0
          for (let run = 0; run < totalRuns; run++) {
            total += newBoard
          }
          const avg = total / 5
          if (mostSteps < avg) {
            best = newBoard
            mostSteps = avg
          }
        }
      }
      Vue.set(this, 'board', best);
    },
    generateNewWasm() {
      const totalBoards = 5
      const totalRuns = 5

      let best = undefined
      let mostSteps = 0
      for (let numberOfBlanks = 45; numberOfBlanks < 50; numberOfBlanks++) {
        for (let boardCount = 0; boardCount < totalBoards; boardCount++) {
          const newBoard = generateWasm(numberOfBlanks)
          let total = 0
          for (let run = 0; run < totalRuns; run++) {
            total += newBoard.solve().steps()
          }
          const avg = total / 5
          if (mostSteps < avg) {
            best = newBoard
            mostSteps = avg
          }
        }
      }
      Vue.set(this, 'board', best);
    }
  },
  data() {
    return {
      board: this.initialBoard
    }
  }
})

setUp().then(() => {
  const numberEmpty = 25
  const board = generateWasm(numberEmpty)
  const app = new Vue({
    el: '#container',
    data: {
      board: board,
      numberEmpty: numberEmpty,
    }
  })
})