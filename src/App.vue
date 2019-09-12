<template>
  <div id="app">
    <DemoEditor :demo.sync="demo"></DemoEditor>
  </div>
</template>

<script>
import DemoEditor from '@/components/DemoEditor.vue'

export default {
  name: 'App',
  data () {
    return {
      demo: null
    }
  },
  components: {
    DemoEditor
  },
  watch: {
    demo () {
      localStorage.demo = JSON.stringify(this.demo)
    }
  },
  mounted () {
    try {
      let demo = JSON.parse(localStorage.demo)
      if (typeof demo !== 'object') {
        throw new Error('Invalid parsed demo object')
      }

      this.demo = demo
    } catch (_error) {
      this.demo = {
        common_code: 'precision mediump float;\nin vec2 texCoords;\nout vec4 color;',
        passes: [
          {
            name: 'image',
            fragment: 'void main() {\n    color = vec4(texCoords.xy, 0.5, 1.0);\n}'
          }
        ]
      }
    }
  }
}
</script>

<style lang="scss">
@import "~bulma/sass/utilities/_all";

@import "~bulma";
@import "~buefy/src/scss/buefy";

html {
  font-size: 18px;
  @include desktop {
    font-size: 12px;
  }
}
</style>
