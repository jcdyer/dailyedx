import React from 'react'
import {render} from 'react-dom'
import $ from 'jquery'

function makeUrl (email) {
  const today = new Date()
  const date = today.toISOString().slice(0, 10)
  const url = '/dailyedx/' + email + '/' + date
  return url
}

class App extends React.Component {
  constructor (props) {
    super(props)
    this.state = {
      'courses': [],
      'units': [],
      'email': 'cdyer@edx.org'
    }
    this.updateEmail = this.updateEmail.bind(this)
  }

  updateEmail (email) {
    this.setState({'email': email})
  }

  render () {
    if (this.state.email) {
      return <Context email={this.state.email} />
    }
    return <EmailInput updateEmail={this.updateEmail} />
  }
}

class Context extends React.Component {
  constructor (props) {
    super(props)
    this.state = {'completed': 'waiting...', 'units': []}
  }

  componentDidMount () {
    const url = makeUrl(this.props.email)
    const self = this
    $.getJSON(url, function (data) {
      self.setState(
        {'completed': data['completed'], 'units': data['units']}
      )
    })
  }

  render () {
    return (
        <div>
          <Welcome email={this.props.email} completed={this.state.completed} />
          <Problems email={this.props.email} units={this.state.units} />
          <Congratulations email={this.props.email} units={this.state.completed} />
        </div>
    )
  }
}

class Welcome extends React.Component {
  render () {
    return (
      <div>
        hi {this.props.email}, you have completed {this.props.completed} units
      </div>
    )
  }
}

class Problems extends React.Component {
  render () {
    console.log(this.props.units)
    const problems = this.props.units.map((problem) => <Problem key={problem} loc={problem} />)
    return <ul> {problems} </ul>
  }
}

class Problem extends React.Component {
  render () {
    return <li> I am a problem of location {this.props.loc} </li>
  }
}

class Congratulations extends React.Component {
  render () {
    return <div> congrats {this.props.email} for completing today's problems! </div>
  }
}

class XBlockView extends React.Component {
  render () {
    return <iframe src={this.props.xblockurl} width="500" height="1000"/>
  }
}

class EmailInput extends React.Component {
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
    this.props.updateEmail(this.state.value)
  }

  render () {
    return (
      <form onSubmit={this.handleSubmit}>
        <label>
          What is your email address?
          <input type="email" value={this.state.value} onChange={this.handleChange} />
        </label>
        <input type="submit" value="Submit" />
      </form>
    )
  }
}

render(<App/>, document.getElementById('app'))
