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
  props: ['board'],
  template: `
    <div class="playarea">
      <board :board="foo"></board>
      <button @click="solve">Solve</button>
      <button @click="generateNew">Generate New</button>
    </div>
  `,
  methods: {
    solve() {
      const result = this.foo.solve()
      Vue.set(this, 'foo', result.game());
    },
    generateNew() {
      const newBoard = generateWasm(25)
      Vue.set(this, 'foo', newBoard);
    }
  },
  data() {
    return {
      foo: this.board
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