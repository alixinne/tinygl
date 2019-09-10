<template>
  <div>
    <canvas ref="canvas" width="640" height="480"></canvas>
  </div>
</template>

<script>
export default {
  name: 'TgCanvas',
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
        this.animationFrameId = window.requestAnimationFrame(() => {
          this.state.render()
        })
      })
      .catch(console.error)
  }
}
</script>

<style scoped lang="scss">
</style>
