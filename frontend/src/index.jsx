import React from 'react';
import {render} from 'react-dom';

class App extends React.Component {
  constructor(props) {
    super(props);
    this.state = {"xblockurl": ""};
  }

  render () {
    return (
        <div>
            <p> Hello React!</p>
            <URLInput />
            <br/>
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

render(<App/>, document.getElementById('app'));
