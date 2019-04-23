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

const totalBoards = 30
const totalRuns = 30
const numberOfBlanks = 51
Vue.component('playarea', {
    props: ['initialBoard'],
    template: `
    <div class="playarea">
      <board :board="board"></board>
      <div class="controls">
        <div class="button solve" @click="solve">Solve</div>
        <div class="generator">
          <div class="button new" @click="generateNewWasm">New</div>
        </div>
      </div>
    </div>
  `,
    methods: {
        solve() {
            const result = this.board.solve()
            Vue.set(this, 'board', result.game());
        },
        newNumBlank(e) {
            const value = e.target.value
            if (value >= 1 && value <= 81) {
                Vue.set(this, 'numBlank', value);
            } else {
                e.target.value = this.numBlank
            }
        },
        generateNewJS() {
            const numBlank = this.numBlank
            const newBoard = log('JS', () => generateJs(numBlank))
            const { Game } = wasm_bindgen;
            Vue.set(this, 'board', Game.from(newBoard));
        },
        generateNewWasm() {
            const numBlank = this.numBlank
            const newBoard = log('Wasm', () => generateWasm(numBlank))
            Vue.set(this, 'board', newBoard);
        }
    },
    data() {
        return {
            board: this.initialBoard,
            numBlank: 50
        }
    }
})

function log(title, perf) {
    const t1 = performance.now()
    const result = perf()
    const t2 = performance.now()

    console.log(`${title}: `, Math.floor(t2 - t1), 'ms')
    return result
}

setUp().then(() => {
    const numberEmpty = 50
    const board = generateWasm(numberEmpty)
    const app = new Vue({
        el: '#container',
        data: {
            board: board,
            numberEmpty: numberEmpty,
        }
    })
})