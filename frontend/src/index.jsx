import React from 'react'
import {render} from 'react-dom'
import $ from 'jquery'
import style from './main.scss'

import Welcome from './welcome.jsx'
import Problems from './problems.jsx'
import Congratulations from './congratulations.jsx'


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
        <div>
          <Problems
            username={this.props.username}
            units={this.state.units}
            updateStage={this.updateStage}/>
        </div>
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
          {"Log in: "}
          <input type="username" value={this.state.value} onChange={this.handleChange} />
        </label>
        <input type="submit" value="Submit" />
      </form>
    )
  }
}


function makeUrl (username) {
  const today = new Date()
  const date = today.toISOString().slice(0, 10)
  const url = '/dailyedx/' + username + '/' + date
  return url
}


render(<App/>, document.getElementById('app'))
