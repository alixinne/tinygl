<template>
  <div id="app">
    <div class="has-background-dark">
      <div class="container">
        <b-navbar type="is-dark">
          <template slot="brand">
            <b-navbar-item><span class="is-size-3 text-bold">tinygl</span></b-navbar-item>
          </template>

          <template slot="start">
          </template>

          <template slot="end">
          </template>
        </b-navbar>
      </div>
    </div>

    <section class="section">
      <div class="container">
        <DemoEditor :demo.sync="demo"></DemoEditor>
      </div>
    </section>
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
        common_code: 'precision mediump float;\nin vec2 texCoords;\nout vec4 color;\nuniform vec3 iResolution;\nuniform float iTime;\nuniform int iFrame;',
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
@import "@/scss/_darkly";
@import "~bulma";
@import "~buefy/src/scss/buefy";

html {
  font-size: 18px;
  @include desktop {
    font-size: 12px;
  }
}
</style>
