<template>
  <div>
    <canvas ref="canvas" width="640" height="480"></canvas>
  </div>
</template>

<script>
import { debounce } from 'lodash-es'

export default {
  name: 'TgCanvas',
  props: {
    demo: null,
    continuous: () => false
  },
  data () {
    return {
      animationFrameId: null,
      state: null
    }
  },
  beforeCreate () {
    import('@/../pkg')
      .then(tinyglRenderer => {
        this.state = tinyglRenderer.init(this.$refs.canvas)

        if (this.demo !== null) {
          // we have a pending demo object
          this.loadDemo()
          this.render()
        }
      })
      .catch(console.error)
  },
  created () {
    window.addEventListener('resize', this.resizeCanvas)
    this.$nextTick(() => this.resizeCanvas())
  },
  destroyed () {
    window.removeEventListener('resize', this.resizeCanvas)
  },
  watch: {
    demo () {
      if (this.state === null) {
        // renderer not ready yet
        return
      }

      this.loadDemo()
    },
    continuous () {
      if (this.continuous && this.animationFrameId !== null) {
        // Start rendering again if continuous turns back on
        this.render()
      }
    }
  },
  methods: {
    loadDemo () {
      try {
        this.state.load_demo(this.demo)
        this.renderOnce()
      } catch (error) {
        this.$buefy.toast.open({
          message: error,
          type: 'is-danger'
        })
      }
    },
    render () {
      this.renderOnce()
      if (this.continuous) {
        this.animationFrameId = window.requestAnimationFrame(this.render)
      } else {
        this.animationFrameId = null
      }
    },
    renderOnce () {
      this.state.render()
    },
    resizeCanvas: debounce(function () {
      let canvas = this.$refs.canvas
      let width =
        window.innerWidth <= 768
          ? window.innerWidth
          : window.innerWidth < 1024
            ? 768
            : window.innerWidth < 1216
              ? 540
              : 640

      canvas.width = width
      canvas.height = width * 10 / 16
    }, 100)
  }
}
</script>

<style scoped lang="scss">
</style>
