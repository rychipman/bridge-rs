import React from "react";
import { inject, observer } from "mobx-react";
import { Link } from "react-router-dom";
import {
    Container,
	Dropdown,
	Image,
	Loader,
	Menu,
	Sidebar,
} from 'semantic-ui-react';
import logo from '../clubs.svg';

const menuContents = ({ rootStore }) => {
    const { currentUser } = rootStore.commonStore;
    if (!currentUser || currentUser === '') {
        return ([
            <Menu.Item as={Link} to='/login' key='login'>
                Log In
            </Menu.Item>,
            <Menu.Item as={Link} to='/register' key='register'>
                Register
            </Menu.Item>,
        ])
    }
    return ([
        <Menu.Item as={Link} to='/bid?next=bid' key='sets'>Bid</Menu.Item>,
        <Menu.Item as={Link} to='/review' key='review'>Review</Menu.Item>,
        <Dropdown item text={currentUser} position='right' key='dropdown'>
            <Dropdown.Menu>
                <Dropdown.Item
                  text='Profile'
                  as={Link}
                  to={'/user/'+currentUser}
                />
                <Dropdown.Item
                  text='Log Out'
                  onClick={() => rootStore.authStore.logout()}
                />
            </Dropdown.Menu>
        </Dropdown>,
    ])
}
const MenuContents = inject('rootStore')(observer(menuContents));

@inject('rootStore')
@observer
class LogoLoader extends React.Component {
	render() {
		const style = { marginRight: '1.5em' };
		const loading = this.props.rootStore.loading;
		if (loading) {
			return <Loader size='medium' active inline style={style} />;
		}
		return <Image size='mini' src={logo} style={style} />;
	}
}

class HeaderLayout extends React.Component {
	render() {
		return ([
			<Menu fixed='top' borderless style={{ minHeight: '5rem' }}>
				<Container>
					<Menu.Item header onClick={() => this.props.onLogoClick()}>
						<LogoLoader loading />
						Bridge Skills
					</Menu.Item>
					<MenuContents />
				</Container>
			</Menu>,
			<Container style={{ marginTop: '7rem' }}>
				{this.props.children}
			</Container>
		]);
	}
}

class Layout extends React.Component {
	state = { sidebar: false }
	handleShow = () => this.setState({sidebar: true});
	handleHide = () => this.setState({sidebar: false});

	render() {
		const { sidebar } = this.state;
		return (
			<Sidebar.Pushable>
				<Sidebar as={Menu} visible={sidebar} animation='overlay' vertical onHide={this.handleHide}>
					<Menu.Item as={Link} to='/review' content='Review' />
					<Menu.Item as={Link} to='/users' content='Users' />
					<Menu.Item as={Link} to='/bid' content='Bid One' />
				</Sidebar>
				<Sidebar.Pusher dimmed={sidebar}>
					<HeaderLayout onLogoClick={this.handleShow}>
						{this.props.children}
					</HeaderLayout>
				</Sidebar.Pusher>
			</Sidebar.Pushable>
		);
	}
}

export default Layout;
