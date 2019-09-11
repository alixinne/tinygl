<template>
  <div id="app">
    <div class="columns">
      <div class="column is-narrow">
        <TgCanvas :demo="currentDemo"></TgCanvas>
      </div>
      <div class="column">
        <b-tabs type="is-boxed">
          <b-tab-item v-for="pass in passes" :key="pass.name" :label="pass.name">
            <codemirror v-model="pass.fragment" :options="cmOptions" v-on:keyHandled="editorKeyHandled"></codemirror>
          </b-tab-item>
        </b-tabs>

        <b-button type="is-primary" v-on:click="playDemo">Launch</b-button>
      </div>
    </div>
  </div>
</template>

<script>
import TgCanvas from './components/TgCanvas.vue'

import { codemirror } from 'vue-codemirror'
import 'codemirror/lib/codemirror.css'
import 'codemirror/mode/clike/clike.js'

import cloneDeep from 'lodash-es'

export default {
  name: 'App',
  data () {
    return {
      passes: [
        {
          name: 'image',
          fragment: 'precision mediump float;\nin vec2 texCoords;\nout vec4 color;\n\nvoid main() {\n    color = vec4(texCoords.xy, 0.5, 1.0);\n}'
        }
      ],
      currentDemo: null,
      cmOptions: {
        tabSize: 4,
        theme: 'default',
        mode: 'x-shader/x-fragment',
        lineNumbers: true,
        lineWrapping: true,
        line: true,
        extraKeys: {
          'Ctrl-Enter': function (_cm) { /* see editorKeyHandled */ }
        }
      }
    }
  },
  components: {
    TgCanvas,
    codemirror
  },
  methods: {
    playDemo () {
      this.currentDemo = cloneDeep({ passes: this.passes })
      localStorage.passes = JSON.stringify(this.passes)
    },
    editorKeyHandled (_instance, name, _event) {
      if (name === 'Ctrl-Enter') {
        this.playDemo()
      }
    }
  },
  mounted () {
    if (localStorage.passes) {
      this.passes = JSON.parse(localStorage.passes)
    }

    this.playDemo()
  }
}
</script>

<style lang="scss">
</style>
