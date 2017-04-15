import React from 'react'
import {render} from 'react-dom'
import $ from 'jquery'
import style from './main.scss'

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

class Welcome extends React.Component {
  render () {
    return (
      <div className="container">
          <p className="row lead">
            Hello {this.props.username}!
          </p>
          <p className="row">
           Do you think you can knock off {this.props.toComplete} units today?
          </p>
        <div className="row">
          <button
            onClick={this.props.updateStage}
            className="btn btn-success">
              Heck yeah I do!
          </button>
        </div>
      </div>
    )
  }
}

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

class Congratulations extends React.Component {
  render () {
    const edxPink = "#c2387d"
    return (
      <div style={{textAlign: "center", fontSize: "200%" }} >
        <h2 style={{color: edxPink, fontSize: "36pt"}}>{"Rockin'!"}</h2>
        <p>You completed {"today's"} assignment!<br/>Come back tomorrow to keep rolling.</p>
        <p>
          <a href="https://www.youtube.com/watch?v=mRf3-JkwqfU" className="btn btn-success">Go watch Puppy Vids</a>
        </p>
      </div>
    )
  }
}

class XBlockView extends React.Component {
  render () {
    return <iframe src={this.props.xblockurl} width="1000" height="500"/>
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

class ProgressBar extends React.Component {
  render () {
    const edxBlue = '#0075b4'
    const edxPink = '#c2387d'
    const edxGray = '#d9d9d9'
    let bars = []
    let i = 0
    for (i = 0; i < this.props.total; i += 1) {
      let color = edxGray
      if (i < this.props.completed) {
        color = edxBlue
      } else if (i === this.props.completed) {
        color = edxPink
      } else {
        color = edxGray
      }
      bars.push(color)
    }
    const width = 480
    const elements = bars.map(
    (fillColor, index) => <span key={index} style={ { backgroundColor: fillColor, color: fillColor, width: (width / this.props.total) + 'px', height: '4px', marginRight: '3px'} } className="progress-tick"> index </span>
  )

    return (
      <div style={ {width: width + 'px', marginBottom: '.5em'} } className='progressbar'>{elements}</div>
    )
  }
}

render(<App/>, document.getElementById('app'))
