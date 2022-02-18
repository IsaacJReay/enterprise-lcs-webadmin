import React, { useState, useEffect } from "react";
import {
  Layout,
  Col,
  Row,
  Select,
  Button,
  Input,
  Form,
  Checkbox,
  Radio,
  message,
  Spin,
} from "antd";
import axios from "axios";

const { Content } = Layout;
const { Option } = Select;

const options = [];

for (let i = 0; i < 14; i++) {
  options.push(<Option value={i + 1}>{i + 1}</Option>);
}

const getToken = localStorage.getItem("token");

const WirelessSetting = () => {
  const auth = {
    Authorization: "Bearer " + getToken,
  };
  // -----state ---------
  const [loading, setLoading] = useState(false);
  const [form] = Form.useForm();
  const layout = {
    labelCol: {
      span: 8,
    },
    wrapperCol: {
      span: 16,
    },
  };

  // ----------get data -------------

  useEffect(async () => {
    setLoading(true);
    await axios({
      method: "GET",
      url: "http://10.42.0.188:8080/private/api/settings/hostapd/status",
      headers: {
        "content-type": "application/json",
        ...auth,
      },
    })
      .then((res) => {
        const {
          ssid,
          hide_ssid,
          hw_mode,
          channel,
          wpa,
          qos,
          passphrase,
          hw_n_mode,
        } = res.data;
        form.setFieldsValue({
          ssid_name: ssid,
          hide_ssid: hide_ssid,
          mode: hw_mode,
          channel: channel,
          version: wpa,
          password: passphrase,
          qos: qos,
          hw_n_mode: hw_n_mode,
        });
        setLoading(false);
        setTimeout(() => {
          setLoading(false);
        }, 1000);
      })
      .catch((err) => console.log(err));
  }, []);

  // ------- apply button ---------

  const handleApply = (data) => {
    const inputData = {
      ssid: data.ssid_name,
      hide_ssid: data.hide_ssid,
      hw_mode: data.mode,
      channel: data.channel,
      wpa: data.version,
      passphrase: data.password,
      qos: data.qos,
      hw_n_mode: data.hw_n_mode,
    };
    axios
      .post("http://10.42.0.188:8080/private/api/settings/hostapd", inputData, {
        headers: {
          "content-type": "application/json",
          ...auth,
        },
      })

      .then((res) => {
        if (res.data.operation_status === "Success") {
          setLoading(true);
          message.success("Successful!");
          setLoading(false);
        } else {
          setLoading(true);
          message.error("Operation Failed! ");
          setLoading(false);
        }
      })
      .catch((err) => console.log(err));
  };

  if (loading) {
    return (
      <div className="spin">
        <Spin />
      </div>
    );
  }

  return (
    <React.Fragment>
      <Content>
        <Row gutter={12}>
          <Col span={16}>
            <Form {...layout} onFinish={handleApply} form={form}>
              <div className="card">
                <div className="container">
                  <div className="container-header">
                    <h1>Wireless Setting</h1>
                  </div>
                  <hr />

                  <div className="desc-container-banner">
                    <Form.Item label="Network Name SSID">
                      <Row gutter={[12, 0]}>
                        <Col>
                          <Form.Item
                            name="ssid_name"
                            rules={[
                              {
                                required: true,
                                message: "SSID name is require!",
                              },
                            ]}
                          >
                            <Input size="large" placeholder="Text here ..." />
                          </Form.Item>
                        </Col>
                        <Col>
                          <Form.Item name="hide_ssid" valuePropName="checked">
                            <Checkbox>Hide SSID</Checkbox>
                          </Form.Item>
                        </Col>
                      </Row>
                    </Form.Item>
                    {/* <Form.Item label="Security " name="security">
                    <Select size="large" className="select-option-wireless">
                      <Option value="1">WPA</Option>
                      <Option value="2">WPA2-Personal (Recommended)</Option>
                    </Select>
                  </Form.Item> */}
                    <Form.Item label="Version " name="version">
                      <Radio.Group valuePropName="checked">
                        <Radio value={1}>WPA-SPK</Radio>
                        <Radio value={2}>WPA2-SPK</Radio>
                      </Radio.Group>
                    </Form.Item>
                    <Form.Item
                      label="Password "
                      name="password"
                      rules={[
                        {
                          required: true,
                          message: "Password is require!",
                        },
                      ]}
                    >
                      <Input.Password
                        size="large"
                        placeholder="password"
                        className="label-info"
                      />
                    </Form.Item>
                    <Form.Item label="Mood" name="mode">
                      <Select size="large" className="select-option-wireless">
                        <Option value="g">g</Option>
                        <Option value="b">b</Option>
                      </Select>
                    </Form.Item>

                    <Form.Item label="Channel" name="channel">
                      <Select size="large" className="select-option">
                        {options}
                      </Select>
                    </Form.Item>
                    <div className="wireless-radios-options">
                      <Form.Item name="qos" valuePropName="checked">
                        <Checkbox value="QOS">QOS</Checkbox>
                      </Form.Item>
                      <Form.Item name="hw_n_mode" valuePropName="checked">
                        <Checkbox value="802.11N">802.11N</Checkbox>
                      </Form.Item>
                    </div>
                  </div>
                  <Form.Item>
                    <Button
                      type="primary"
                      htmlType="submit"
                      className="button-apply"
                      size="large"
                    >
                      Apply
                    </Button>
                  </Form.Item>
                </div>
              </div>
            </Form>
          </Col>
          <Col span={8}>
            <div className="card">
              <div className="container">
                <div className="container-header">
                  <h1>Desciptions</h1>
                </div>
                <div>
                  <h2>Wireless Settings Help</h2>
                  <p>
                    <strong>Note:</strong> The operating distance or range of
                    your wireless connection varies significantly based on the
                    physical placement of the Router. For best results, place
                    your Router.
                  </p>
                  <ul>
                    <li>
                      Near the center of the area in which your wireless
                      stations will operate.
                    </li>
                    <li>In an elevated location such as a high shelf.</li>
                    <li>
                      Away from the potential sources of interference, such as
                      PCs, microwaves, and cordless phones.
                    </li>
                    <li>With the Antenna in the upright position.</li>
                    <li>Away from large metal surfaces.</li>
                  </ul>
                  <p>
                    <strong>Note:</strong> Failure to follow these guidelines
                    can result in significant performance degradation or
                    inability to wirelessly connect to the Router.
                  </p>
                  <ul>
                    <li>
                      <strong>Wireless Network Name</strong> - Enter a value of
                      up to 32 characters. The same Name (SSID) must be assigned
                      to all wireless devices in your network.
                    </li>
                    <li>
                      <strong>Version</strong> - You can select one of following
                      versions
                    </li>
                    <li>
                      <strong> Password</strong> - Enter the password for the
                      Radius Server.
                    </li>
                    <li>
                      <strong>Channel</strong> - This field determines which
                      operating frequency will be used. It is not necessary to
                      change the wireless channel unless you notice interference
                      problems with another nearby access point. If you select
                      auto, then AP will choose the best channel automatically.
                    </li>
                    <li>
                      <strong>MODE</strong> - If all of the wireless devices
                      connected with this wireless router can connect in the
                      same transmission mode (e.g. 802.11b), you can choose
                      "Only" mode (e.g. 11b only). If you have some devices that
                      use a different transmission mode, choose the appropriate
                      "Mixed" mode.
                    </li>
                  </ul>
                </div>
              </div>
            </div>
          </Col>
        </Row>
      </Content>
    </React.Fragment>
  );
};

export default WirelessSetting;
