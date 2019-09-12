<template>
  <div>
    <div class="columns is-desktop">
      <div class="column">
        <TgCanvas :demo="demo"></TgCanvas>
      </div>
      <div class="column">
        <b-tabs type="is-boxed" v-on:change="codeTabChange" :animated="false" v-if="demo">
          <!-- Common code -->
          <b-tab-item label="Common">
            <codemirror ref="editorCommon" class="tab-glsl-editor" v-model="demo.common_code" :options="cmOptions" v-on:keyHandled="editorKeyHandled"></codemirror>
          </b-tab-item>
          <!-- Demo passes -->
          <b-tab-item v-for="pass in demo.passes" :key="pass.name" :label="pass.name | titlecase">
            <codemirror ref="editorPass" class="tab-glsl-editor" v-model="pass.fragment" :options="cmOptions" v-on:keyHandled="editorKeyHandled"></codemirror>
          </b-tab-item>
        </b-tabs>

        <b-button type="is-primary" v-on:click="playDemo">Launch</b-button>
      </div>
    </div>
  </div>
</template>

<script>
import TgCanvas from '@/components/TgCanvas.vue'

import { codemirror } from 'vue-codemirror'
import 'codemirror/lib/codemirror.css'
import 'codemirror/mode/clike/clike.js'

export default {
  name: 'DemoEditor',
  props: {
    demo: null
  },
  data () {
    return {
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
      // Stringify duplicate to turn reactive object into a data object
      this.$emit('update:demo', JSON.parse(JSON.stringify(this.demo)))
    },
    editorKeyHandled (_instance, name, _event) {
      if (name === 'Ctrl-Enter') {
        this.playDemo()
      }
    },
    codeTabChange (index) {
      let editorTarget = index === 0
        ? this.$refs.editorCommon
        : this.$refs.editorPass[index - 1]
      editorTarget.refresh()
      this.$nextTick(() => editorTarget.cminstance.focus())
    }
  }
}
</script>

<style lang="scss" scoped>
.tab-glsl-editor {
  margin: -1rem;
}
</style>
