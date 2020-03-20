
const suitSymbols = {
    N: 'NT',
    S: '♠',
    H: '♥',
    D: '♦',
    C: '♣',
}

const suitSymbol = (suit) => {
    const key = suit.toUpperCase().slice(0, 1)
    return suitSymbols[key]
}

const suitStyle = (suit) => {
    const key = suit.toUpperCase().slice(0, 1)
    if (key === 'N') {
        return {}
    }
	const colorByKey = {
		'S': '#0000ff',
		'H': '#ff0000',
		'D': '#ff8800',
		'C': '#008800',
	}
    return {
        color: colorByKey[key],
        fontSize: '1.3em',
    }
}

const parseBid = (bidStr) => {
    let str = bidStr.toUpperCase()

    let special = ''
    switch (str.slice(0, 1)) {
        case 'P':
            special = 'Pass'
            break
        case 'D':
            special = 'Dbl'
            break
        case 'R':
            special = 'Rdbl'
            break
        default:
            // do nothing
    }

    if (special !== '') {
        return {
            special: special,
            level: 0,
            suit: '',
        }
    }

    return {
        level: parseInt(str.slice(0, 1), 10),
        suit: str.slice(1, 2),
        special: '',
    }
}

const bidToString = (bid) => {
    if (!bid) {
        return undefined
    }

    switch (bid.special) {
    case 'Pass':
        return 'P'
    case 'Dbl':
        return 'Dbl'
    case 'Rdbl':
        return 'Rdbl'
    default:
        // do nothing
    }
    if (bid.suit === 'N') {
        return '' + bid.level + 'NT'
    }
    return '' + bid.level + bid.suit
}

const compareSuit = (suit, other) => {
    if (suit === other) {
        return 0
    } else if (suit === 'N') {
        return 1
    } else if (other === 'N') {
        return -1
    }
    return suit > other ? 1 : -1
}

const compareBid = (bid, other) => {
    if (bid.special !== '' || other.special !== '') {
        return 0
    }

    if (bid.level !== other.level) {
        return bid.level > other.level ? 1 : -1
    }

    return compareSuit(bid.suit, other.suit)
}

const lastNonPassBid = (bids) => {
    for (let i in bids) {
        const idx = bids.length-i-1
        const bid = bids[idx]
        if (bid.special !== 'Pass') {
            return idx
        }
    }
    return -1
}

const lastSuitedBid = (bids) => {
    for (let i in bids) {
        const idx = bids.length-i-1
        const bid = bids[idx]
        if (bid.special === '') {
            return idx
        }
    }
    return -1
}

const bidIsLegal = (bid, pastBids=[]) => {
    if (biddingFinished(pastBids)) {
        return false
    }

    if (typeof bid !== 'object') {
        bid = parseBid(bid)
    }

    const lastNonPass = lastNonPassBid(pastBids)
    const lastSuited = lastSuitedBid(pastBids)

    if (lastNonPass === -1) {
        switch (bid.special) {
        case 'Dbl':
        case 'Rdbl':
            return false
        default:
            return true
        }
    }

    switch (bid.special) {
    case 'Pass':
        return true
    case 'Dbl':
        return lastNonPass >= 0
            && lastNonPass === lastSuited
            && (pastBids.length - lastSuited) % 2 === 1
    case 'Rdbl':
        return lastNonPass >= 0
            && pastBids[lastNonPass].special === 'Dbl'
            && (pastBids.length - lastNonPass) % 2 === 1
    default:
        // switch is exhaustive
    }

    return compareBid(bid, pastBids[lastSuited]) === 1
}

const biddingFinished = (bids) => {
    if (typeof bids[0] !== 'object') {
        bids = bids.map(b => parseBid(b))
    }

    if (bids.length < 4) {
        return false
    }

    const lastThree = bids.slice(bids.length-3)

    const includesNonPass = lastThree.reduce(
        (acc, bid) => bid.special !== 'Pass' || acc,
        false,
    )

    return !includesNonPass
}

const nextBid = (dealer, bids) => {
    if (biddingFinished(bids)) {
        return null
    }

    const seats = ['north', 'east', 'south', 'west']
    const dealerIdx = seats.findIndex(seat => seat === dealer)
    if (dealerIdx === -1) {
        throw new Error('could not find dealer')
    }

    const nextIdx = (dealerIdx + bids.length) % 4
    return seats[nextIdx]
}

const shortRanks = {
	Ace: 'A',
	King: 'K',
	Queen: 'Q',
	Jack: 'J',
	Ten: 'T',
	Nine: '9',
	Eight: '8',
	Seven: '7',
	Six: '6',
	Five: '5',
	Four: '4',
	Three: '3',
	Two: '2',
};

const shortenRankString = (rank) => shortRanks[rank]

module.exports = {
    suitSymbol,
    suitStyle,
	bidIsLegal,
	shortenRankString,
	parseBid,
	bidToString,
	nextBid,
	biddingFinished,
}
