import React from 'react';
import { Table } from 'semantic-ui-react';
import { suitStyle, suitSymbol } from '../util/bridge';

const bidToComponent = (bid) => {
    if (!bid) {
        return null
    }

    if (typeof bid !== 'object') {
        return bid
    }

    if (bid.special !== '') {
        return bid.special
    }

    return (
        <span>
            {bid.level}
            <span style={suitStyle(bid.suit)}>{suitSymbol(bid.suit)}</span>
        </span>
    )
}

const paginate = (input) => {
    let list = input.slice(0)

    let out = []
    let begin = 0
    let end = 4

    while (true) {
        if (end >= list.length) {
            let row = list.slice(begin, list.length)
            while (row.length < 4) {
                row.push(null)
            }
            out.push(row)
            break
        }
        let row = list.slice(begin, end)
        out.push(row)
        begin = end
        end += 4
    }

    return out
}

const HeaderCell = ({ seat, vulnerable }) => {
    if (vulnerable) {
        vulnerable = vulnerable.toUpperCase()
    }

    let vuln = false
    switch (seat) {
    case 'North':
    case 'South':
        vuln = vulnerable === 'BOTH' || vulnerable === 'NS'
        break
    case 'East':
    case 'West':
        vuln = vulnerable === 'BOTH' || vulnerable === 'EW'
        break
    default:
        // nothing to do
    }

    let content = seat
    if (vuln) {
        content += '*'
    }

    return <Table.HeaderCell content={content} />
};

const paddingBySeat = {
    north: 0,
    east:  1,
    south: 2,
    west:  3,
};

const BidTable = ({ bids, nextBid, dealer, vulnerable }) => {
    if (nextBid) {
		bids = bids.concat([nextBid]);
	} else {
        bids = bids.concat(['?']);
    }

    const padding = (paddingBySeat[dealer]) % 4;
    for (let i=0; i<padding; i++) {
        bids.unshift('');
    }

    return (
        <Table unstackable celled textAlign='center'>
            <Table.Header>
                <Table.Row>
                {['North', 'East', 'South', 'West'].map(seat => (
                    <HeaderCell
                      key={seat}
                      seat={seat}
                      vulnerable={vulnerable}
                    />
                ))}
                </Table.Row>
            </Table.Header>
            <Table.Body>
            {paginate(bids).map((row, idx) => (
                <Table.Row key={idx}>
                {row.map((bid, idx) => (
                    <Table.Cell
                      key={idx}
                      content={bidToComponent(bid)} />
                ))}
                </Table.Row>
            ))}
            </Table.Body>
        </Table>
    )
}

export default BidTable;
