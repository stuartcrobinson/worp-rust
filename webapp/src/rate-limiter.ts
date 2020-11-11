export class RateLimiter {
  private lastRunCompleteTime = 0
  private delayInMillis: number
  private handler?: (immediate?: boolean) => void | Promise<void>
  private nextRunTimer:  NodeJS.Timeout | undefined
  private immediateIsQueued = false
  private onlyHandleImmediates?: boolean
  private isRunningImmediate = false

  constructor(options: {
    delayInMillis: number
    handler?: (immediate?: boolean) => void | Promise<void>
    onlyHandleImmediates?: boolean
  }) {
    this.delayInMillis = options.delayInMillis
    this.handler = options.handler
    this.onlyHandleImmediates = options.onlyHandleImmediates
  }

  clear() {
    this.immediateIsQueued = false
    if (this.nextRunTimer !== undefined) {
      clearTimeout(this.nextRunTimer)
      this.nextRunTimer = undefined
    }
  }

  destroy() {
    this.clear()
  }

  private async _run(immediate?: boolean) {
    if (this.handler) {
      if (immediate || !this.onlyHandleImmediates) {
        if (immediate) {
          this.clear()
          this.isRunningImmediate = true
        }
        try {
          await this.handler(immediate)
        } catch (e) {
          console.error(e)
        }
        if (immediate) {
          this.lastRunCompleteTime = Date.now()
          this.isRunningImmediate = false
          if (this.immediateIsQueued) {
            this.immediateIsQueued = false
            this.run({immediate: true})
          }
        }
      }
    }
  }

  async run(options?: {
    handler?: (immediate?: boolean) => void
    immediate?: boolean
  }) {
    const optionsConst = options || {}
    if (optionsConst.handler) {
      this.handler = optionsConst.handler
    }
    if (!optionsConst.immediate || this.isRunningImmediate) {
      const millisSinceLastRender = Date.now() - this.lastRunCompleteTime
      const waitTimeMillis = Math.max(0, this.delayInMillis - millisSinceLastRender)
      if (waitTimeMillis > 0 || this.isRunningImmediate) {
        if (this.nextRunTimer === undefined && !this.immediateIsQueued) {
          if (this.isRunningImmediate) {
            this.immediateIsQueued = true
          } else {
            this.nextRunTimer = setTimeout(() => this.run({immediate: true}), waitTimeMillis)
          }
        }
        await this._run(false)
        return false
      }
    }
    await this._run(true)
    return true
  }
}