import React, { useState, useEffect } from "react";
import { Row, Col, Layout, Radio, Form } from "antd";
import Automaticaly from "./auto";
import CustomeTime from "./manaul";
import axios from "axios";

const { Content } = Layout;

const TimeSetting = () => {
  const [pick, setPick] = useState();
  const [, setLoading] = useState(false);
  const [items, setItems] = useState({});
  const [form] = Form.useForm();

  // ------token ------

  const getToken = localStorage.getItem("token");
  const auth = {
    Authorization: "Bearer " + getToken,
  };

  const handleChange = (e) => {
    return setPick(e.target.value);
  };

  // ----------get data -------------

  useEffect(async () => {
    await axios({
      method: "GET",
      url: "http://10.42.0.188:8080/private/api/settings/time/status",
      headers: {
        "content-type": "application/json",
        ...auth,
      },
    })
      .then((res) => {
        setLoading(true);
        const { ntp_status } = res.data;
        form.setFieldsValue({ ntp_status: ntp_status });
        setPick(res.data.ntp_status);
        setItems(res.data);
        setTimeout(() => {
          setLoading(false);
        }, 1000);
      })
      .catch((err) => console.log(err));
  }, []);

  return (
    <React.Fragment>
      <Content>
        <Row gutter={12}>
          <Col span={16}>
            <div className="card">
              <div className="container">
                <div className="container-header">
                  <h1>Time Settings</h1>
                </div>
                <hr />
                <div className="desc-container-banner">
                  <Form form={form}>
                    <Form.Item name="ntp_status">
                      <Radio.Group
                        onChange={handleChange}
                        valuePropName="checked"
                      >
                        <Radio value={true}>
                          Automatically synchronize with an Internet time server
                        </Radio>
                        <Radio className="custom-radio" value={false}>
                          Custom
                        </Radio>
                      </Radio.Group>
                    </Form.Item>
                  </Form>
                  <div className="time-container">
                    <Automaticaly pick={pick} items={items} />
                  </div>
                  <div className="time-container2">
                    <CustomeTime pick={pick} items={items} />
                  </div>
                </div>
              </div>
            </div>
          </Col>
          <Col span={8}>
            <div className="card">
              <div className="container">
                <div className="container-header">
                  <h1>Desciptions</h1>
                </div>
                <div>
                  <h1>Time Settings Help</h1>
                  <p>
                    his page allows you to set the time manually or to configure
                    automatic time synchronization. The Router can automatically
                    update the time from an NTP server via the Internet.
                  </p>
                  <p>
                    <strong>Time Zone</strong> - Select your local time zone
                    from this pull-down list.
                  </p>
                  <p>To set time manually:</p>
                  <ul>
                    <li>Select your local time zone.</li>
                    <li>Enter the Date in Month/Day/Year format.</li>
                    <li>Enter the Time in Hour/Minute/Second format.</li>
                    <li>Click Save.</li>
                  </ul>
                  <p>For automatic time synchronization:</p>
                  <ul>
                    <li>
                      Enter the address or domain of the NTP Server 1 or NTP
                      Server 2.
                    </li>
                    <li>
                      Click the Get GMT button to get GMT from the Internet.
                    </li>
                  </ul>
                  <p>To set up daylight saving:</p>
                  <ul>
                    <li>
                      Select the Enable Daylight Saving checkbox to enable
                      daylight saving function.
                    </li>
                    <li>
                      Select the correct Start time and End time of daylight
                      saving range.
                    </li>
                    <li>Click Save.</li>
                  </ul>
                  <p>
                    <strong>Note:</strong> This setting will be used for some
                    time-based functions such as firewall functions. These time
                    dependant functions will not work if time is not set.
                    Therefore, it is important to specify time settings as soon
                    as you successfully login to the Router. The time will be
                    lost if the Router is turned off.
                  </p>
                  <p>
                    The Router will automatically obtain GMT from the Internet
                    if it is configured accordingly.
                  </p>
                  <p>
                    In daylight saving configuration, start time shall be
                    earlier than end time.
                  </p>
                </div>
              </div>
            </div>
          </Col>
        </Row>
      </Content>
    </React.Fragment>
  );
};

export default TimeSetting;
