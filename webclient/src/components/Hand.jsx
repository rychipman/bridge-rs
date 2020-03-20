import React from 'react'
import { suitStyle, suitSymbol } from '../util/bridge'
import {
    Segment,
} from 'semantic-ui-react'

const style = {
    fontSize: '1.2em',
}

const Hand = ({ cards }) => (
    <Segment style={style}>
    {['Spades', 'Hearts', 'Diamonds', 'Clubs'].map(suit => (
        <p key={suit}>
            <span style={suitStyle(suit)}>{suitSymbol(suit) + " "}</span>
            {cards[suit].reduce((res, val) => res + val, '')}
        </p>
    ))}
    </Segment>
)

export default Hand
