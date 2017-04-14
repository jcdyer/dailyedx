import React from 'react'

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

export default Congratulations
