import React from "react";
import { Col, Row } from "antd";
import history from "../assets/images/history.png";
import mission from "../assets/images/mission.png";
import vision from "../assets/images/vision.png";

const AboutUs = () => {
  const Banner = () => {
    return (
      <React.Fragment>
        <center>
          <div className="about-us-title ">About Us</div>
          <h4 className="about-us-sub-title">CONTENT SERVER</h4>

          <div className="intro-banner">
            <p>
              RACHEL (Remote Area Community Hotspot for Education and Learning)
              is a portable, battery-powered, device that contains copies of
              educational websites in offline format. This means RACHEL can go
              anywhere in the world and wirelessly deliver free digital
              educational content to nearby tablets, laptops, or smartphones
              with no internet or data plans required. RACHEL has been taken to
              over 53 countries since its creation, serving students in rural
              villages, townships, and even prisons.
            </p>
          </div>
        </center>
        <div className="wrap-intro-about-us">
          <Row gutter={[24, 24]}>
            <Col xs={24} sm={24} md={24} lg={8} xl={8}>
              <div className="mission-background">
                <img
                  src={mission}
                  alt="history"
                  className="logo_miss_vis_his"
                />
                <h1 className="mission-header">Mission</h1>
                <p className="mission-p">
                  Building tools and providing resources for the next generation
                  of innovators.
                </p>
              </div>
            </Col>
            <Col xs={24} sm={24} md={24} lg={8} xl={8}>
              <div className="vision-background">
                <img src={vision} alt="history" className="logo_miss_vis_his" />
                <h1 className="vision-header">Vision</h1>
                <p className="mission-p">
                  Unlocking our unlimited capacity for self-directed learning
                  through encouraging curiosity, flexibility, and creative
                  imagination.
                </p>
              </div>
            </Col>
            <Col xs={24} sm={24} md={24} lg={8} xl={8}>
              <div className="history-background">
                <img
                  src={history}
                  alt="history"
                  className="logo_miss_vis_his"
                />
                <h1 className="history-header">History</h1>
                <p className="mission-p">
                  KOOMPI began with a vision of creating equal learning
                  opportunities for students unable to afford an internet
                  enabled computer of their own.
                </p>
              </div>
            </Col>
          </Row>
        </div>
      </React.Fragment>
    );
  };

  return (
    <React.Fragment>
      <div className="container">
        <Banner />
      </div>
    </React.Fragment>
  );
};

export default AboutUs;
