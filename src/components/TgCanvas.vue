<template>
  <div>
    <canvas ref="canvas" width="640" height="480"></canvas>
  </div>
</template>

<script>
export default {
  name: 'TgCanvas',
  props: {
    demo: null
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
  watch: {
    demo () {
      if (this.state === null) {
        // renderer not ready yet
        return
      }

      this.loadDemo()
    }
  },
  methods: {
    loadDemo () {
      try {
        this.state.load_demo(this.demo)
      } catch (error) {
        this.$buefy.toast.open({
          message: error,
          type: 'is-danger'
        })
      }
    },
    render () {
      this.renderOnce()
      this.animationFrameId = window.requestAnimationFrame(this.render)
    },
    renderOnce () {
      this.state.render()
    }
  }
}
</script>

<style scoped lang="scss">
</style>
