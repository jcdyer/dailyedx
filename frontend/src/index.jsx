import React from 'react';
import {render} from 'react-dom';

class App extends React.Component {
  constructor(props) {
    super(props);
    this.state = {"xblockurl": ""}
    this.updateXBlockUrl = this.updateXBlockUrl.bind(this);
  }

  updateXBlockUrl (xblockurl) {

    this.setState({"xblockurl": xblockurl});
  }

  render () {
    return (
        <div>
            <p> Hello React!</p>
            <UrlInput updateXBlockUrl={this.updateXBlockUrl} />
            <XBlockView xblockurl={this.state.xblockurl}/>
        </div>
    );
  }
}

class XBlockView extends React.Component {
    render() {
        return <iframe src={this.props.xblockurl} width="500" height="1000"/>
    }
}

class UrlInput extends React.Component {
  constructor(props) {
    super(props);
    this.state = {value: ''};

    this.handleChange = this.handleChange.bind(this);
    this.handleSubmit = this.handleSubmit.bind(this);
  }

  handleChange(event) {
    this.setState({value: event.target.value});
  }

  handleSubmit(event) {
    event.preventDefault();
    this.props.updateXBlockUrl(this.state.value);
  }

  render() {
    return (
      <form onSubmit={this.handleSubmit}>
        <label>
          Xblock Url:
          <input type="url" value={this.state.value} onChange={this.handleChange} />
        </label>
        <input type="submit" value="Submit" />
      </form>
    );
  }
}

render(<App/>, document.getElementById('app'));
