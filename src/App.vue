<template>
  <div id="app">
    <div class="columns">
      <div class="column is-narrow">
        <TgCanvas :demo="currentDemo"></TgCanvas>
      </div>
      <div class="column">
        <b-tabs type="is-boxed">
          <!-- Common code -->
          <b-tab-item label="Common">
            <codemirror class ="tab-glsl-editor" v-model="demo.common_code" :options="cmOptions" v-on:keyHandled="editorKeyHandled"></codemirror>
          </b-tab-item>
          <!-- Demo passes -->
          <b-tab-item v-for="pass in demo.passes" :key="pass.name" :label="pass.name | titlecase">
            <codemirror class="tab-glsl-editor" v-model="pass.fragment" :options="cmOptions" v-on:keyHandled="editorKeyHandled"></codemirror>
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
      demo: {
        common_code: 'precision mediump float;\nin vec2 texCoords;\nout vec4 color;',
        passes: [
          {
            name: 'image',
            fragment: 'void main() {\n    color = vec4(texCoords.xy, 0.5, 1.0);\n}'
          }
        ]
      },
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
      this.currentDemo = cloneDeep(this.demo)
      localStorage.demo = JSON.stringify(this.demo)
    },
    editorKeyHandled (_instance, name, _event) {
      if (name === 'Ctrl-Enter') {
        this.playDemo()
      }
    }
  },
  mounted () {
    if (localStorage.demo) {
      this.demo = JSON.parse(localStorage.demo)
    }

    this.playDemo()
  }
}
</script>

<style lang="scss">
</style>
