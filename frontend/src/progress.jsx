import React from 'react'
import {render} from 'react-dom'

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

export default ProgressBar
