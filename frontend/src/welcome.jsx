import React from 'react'

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

export default Welcome
