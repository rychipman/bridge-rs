import React from 'react'
import { inject, observer } from 'mobx-react';
import {
    Button,
    Segment,
} from 'semantic-ui-react'
import {
    bidIsLegal,
    suitStyle,
    suitSymbol,
} from '../util/bridge'

class BidChooser extends React.Component {
    constructor(props) {
        super(props)
        this.state = { suit: '', level: 0, special: '' }
    }

    _onLevelClick(lvl) {
        this.setState({ level: lvl, suit: '' })
    }

    _onSuitClick(suit) {
        this.setState({ suit: suit }, () => this._onSubmit())
    }

    _onSpecialClick(special) {
        this.setState({ suit: '', level: 0, special: special }, () => this._onSubmit())
    }

    _onSubmit() {
        this.props.onSubmitBid(this.state)
        this.setState({ suit: '', level: 0, special: '' })
    }

    _isBidLegal(bid) {
        return bidIsLegal(bid, this.props.pastBids)
    }

    render() {
        return (
            <Segment.Group compact>
                {this.renderSpecialChooser()}
                {this.renderLevelChooser()}
                {this.renderSuitChooser()}
            </Segment.Group>
        )
    }

    renderSpecialChooser() {
        return (
            <Segment textAlign='center'>
                {['Pass', 'Dbl', 'Rdbl'].map(bid => (
                    <Button
                      key={bid}
                      content={bid}
                      disabled={this.props.disabled || !this._isBidLegal(bid)}
                      onClick={() => this._onSpecialClick(bid)}
                    />
                ))}
            </Segment>
        )
    }

    renderLevelChooser() {
        return (
            <Segment textAlign='center'>
                {[1, 2, 3, 4, 5, 6, 7].map(lvl => (
                    <Button
                      key={lvl}
                      content={lvl}
                      active={this.state.level === lvl}
                      disabled={this.props.disabled || !this._isBidLegal(`${lvl}NT`)}
                      onClick={() => this._onLevelClick(lvl)}
                    />
                ))}
            </Segment>
        )
    }

    renderSuitChooser() {
        const hasLevel = this.state.level > 0
        return (
            <Segment textAlign='center'>
                {['N', 'S', 'H', 'D', 'C'].map(suit => (
                    <Button
                      key={suit}
                      disabled={this.props.disabled || !hasLevel || !this._isBidLegal('' + this.state.level + suit)}
                      active={this.state.suit === suit}
                      onClick={() => this._onSuitClick(suit)}
                    >
                        <span style={suitStyle(suit)}>{suitSymbol(suit)}</span>
                    </Button>
                ))}
            </Segment>
        )
    }
}

@inject('rootStore')
@observer
class BidChooserContainer extends React.Component {
	exercise() {
		const store = this.props.rootStore.exerciseStore;
		const exercise = store.getExercise(this.props.exercise_id);
		return exercise;
	}

	makeBid(bid) {
		const exercise = this.exercise();
		this.props.rootStore.setLoading(true);
		exercise.bid(bid)
			.then(bid_id => this.props.onBidCreated(bid_id))
			.finally(() => this.props.rootStore.setLoading(false)) ;
	}

	render() {
		const exercise = this.exercise();
		if (!exercise.hasLoaded) {
			return null;
		}
		return (
			<BidChooser
			  disabled={exercise.completed}
			  pastBids={exercise.bids}
			  onSubmitBid={bid => this.makeBid(bid)} />
		);
	}
};

export default BidChooserContainer;
