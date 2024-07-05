import React from 'react';
import { BrowserRouter as Router, Route } from 'react-router-dom';
import Home from './pages/Home/Home';
import Map from './pages/Home/Home';

export const Routes = () => {
  return (
    <Router>
      <Route component={Home} path="/" exact />
      <Route component={Map} path="/map" />
    </Router>
  );
};
