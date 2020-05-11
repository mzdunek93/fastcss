export default class Timer {
  #startTime: number = new Date().getTime()
  #name?: string
  constructor(name: string) {
    this.#name = name
  }
  end() {
    const endTime = new Date().getTime()
    const seconds = (endTime - this.#startTime) / 1000
    console.log(`${this.#name ?? 'Time'}: ${seconds.toFixed(3)}s`)
  }
}