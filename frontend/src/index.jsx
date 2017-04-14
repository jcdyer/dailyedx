import React from 'react'
import {render} from 'react-dom'
import $ from 'jquery'

function makeUrl (username) {
  const today = new Date()
  const date = today.toISOString().slice(0, 10)
  const url = '/dailyedx/' + username + '/' + date
  return url
}

class App extends React.Component {
  constructor (props) {
    super(props)
    this.state = {'username': ''}
    this.updateUsername = this.updateUsername.bind(this)
  }

  updateUsername (username) {
    this.setState({'username': username})
  }

  render () {
    if (this.state.username) {
      return <Context username={this.state.username} />
    }
    return <UsernameInput updateUsername={this.updateUsername} />
  }
}

class Context extends React.Component {
  constructor (props) {
    super(props)
    this.state = {
      'completed': 'waiting...',
      'units': [],
      'stage': 0
    }
    this.updateStage = this.updateStage.bind(this)
  }

  updateStage () {
    this.setState({'stage': this.state.stage + 1})
  }

  componentDidMount () {
    const url = makeUrl(this.props.username)
    const self = this
    $.getJSON(url, function (data) {
      self.setState(
        {'completed': data['completed'], 'units': data['units']}
      )
    })
  }

  render () {
    if (this.state.stage == 0) {
      return (
        <Welcome
          username={this.props.username}
          toComplete={this.state.units.length}
          updateStage={this.updateStage}/>
      )
    } else if (this.state.stage == 1) {
      return (
        <Problems
          username={this.props.username}
          units={this.state.units}
          updateStage={this.updateStage}/>
      )
    } else {
      return (
        <Congratulations
          username={this.props.username}
          units={this.state.completed}/>
      )
    }
  }
}

class Welcome extends React.Component {
  render () {
    return (
      <div>
        hi {this.props.username}, you have to complete {this.props.toComplete} units
        <button onClick={this.props.updateStage} />
      </div>
    )
  }
}

class Problems extends React.Component {
  constructor(props) {
    super(props)
    this.state = {'activeProblem': 0}
    this.goToNextProblem = this.goToNextProblem.bind(this)
  }

  goToNextProblem () {
    this.setState({'activeProblem': this.state.activeProblem + 1})
  }

  render () {
    const problems = this.props.units.map((problem, index) =>
      <Problem
        key={problem}
        loc={problem}
        active={index == this.state.activeProblem}
        goToNextProblem={this.goToNextProblem}/>
    )
    return <ul> {problems} </ul>
  }
}

class Problem extends React.Component {
  render () {
    console.log(this.props.index)
    if (this.props.active) {
      return (
        <li>
          I am a problem of location {this.props.loc}
          <XBlockView xblockurl={'https://courses.edx.org/xblock/' + this.props.loc} />
          <button onClick={this.props.goToNextProblem}>NEXT</button>
        </li>
      )
    }
    return <li> not rendering {this.props.loc}</li>
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

class Congratulations extends React.Component {
  render () {
    return <div> congrats {this.props.username} for completing {"today's"} problems! </div>
  }
}

class XBlockView extends React.Component {
  render () {
    return <iframe src={this.props.xblockurl} width="500" height="1000"/>
  }
}

class UsernameInput extends React.Component {
  constructor (props) {
    super(props)
    this.state = {value: ''}

    this.handleChange = this.handleChange.bind(this)
    this.handleSubmit = this.handleSubmit.bind(this)
  }

  handleChange (event) {
    this.setState({value: event.target.value})
  }

  handleSubmit (event) {
    event.preventDefault()
    this.props.updateUsername(this.state.value)
  }

  render () {
    return (
      <form onSubmit={this.handleSubmit}>
        <label>
          What is your username?
          <input type="username" value={this.state.value} onChange={this.handleChange} />
        </label>
        <input type="submit" value="Submit" />
      </form>
    )
  }
}

render(<App/>, document.getElementById('app'))
