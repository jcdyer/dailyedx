import React from "react"

import ProgressBar from './progress.jsx'


class Problems extends React.Component {
  constructor (props) {
    super(props)
    this.state = {'activeProblem': 0}
    this.goToNextProblem = this.goToNextProblem.bind(this)
  }

  goToNextProblem () {
    const next = this.state.activeProblem + 1
    if (next < this.props.units.length) {
      this.setState({'activeProblem': this.state.activeProblem + 1})
    } else {
      this.props.updateStage()
    }
  }

  render () {
    const problem = this.props.units[this.state.activeProblem]

    return (
      <div>
        <ProgressBar completed={this.state.activeProblem} total={this.props.units.length}/>
        <Problem
          key={problem}
          loc={problem}
          goToNextProblem={this.goToNextProblem}/>
      </div>
    )
  }
}

class Problem extends React.Component {
  render () {
    return (
      <div>
        <XBlockView xblockurl={'https://courses.edx.org/xblock/' + this.props.loc} />
        <button className="btn btn-success" onClick={this.props.goToNextProblem}>Next Unit</button>
      </div>
    )
  }

  handleClick () {
    console.log(this.props.loc)
    toggleActive()
  }

  toggleActive () {
    if (this.state.active) {
      this.setState({'active': false})
    } else {
      this.setState({'active': true})
    }
  }
}


class XBlockView extends React.Component {
  render () {
    return <iframe src={this.props.xblockurl} width="1000" height="500"/>
  }
}


export default Problems
