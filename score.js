class Score {
  constructor() {
    this.scores = {};
  }

  addScore(username) {
    if (this.scores[username]) {
      this.scores[username] += 1;
    } else {
      this.scores[username] = 1;
    }
  }
}
