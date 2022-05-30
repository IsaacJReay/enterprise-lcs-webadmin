import React from "react";
import { Form, Input, Button, message, Row, Col } from "antd";
import axios from "axios";
import Cookies from "js-cookie";

// const getToken = localStorage.getItem("token");
const getToken = Cookies.get("token");
const baseUrl = process.env.REACT_APP_API_URL;

const WANDynamic = ({ wan, fetchData }) => {
  const [form] = Form.useForm();

  const auth = {
    Authorization: "Bearer " + getToken,
  };

  const layout = {
    labelCol: {
      span: 8,
    },
    wrapperCol: {
      span: 16,
    },
  };

  const { internet_ip, gateway, netmask, dns } = wan;

  const handleApply = async () => {
    await axios({
      method: "POST",
      url: `${baseUrl}/settings/wirednetwork/dynamic`,
      headers: {
        "content-type": "application/json",
        ...auth,
      },
    })
      .then(async (res) => {
        if ((res.statusCode = 200)) {
          message.success("Successful!");
          fetchData();
        } else {
          message.error("operation failed! ");
        }
      })

      .catch((err) => {
        console.log(err);
      });
  };

  // if (loading) {
  //   return (
  //     <div className="spin">
  //       <Spin />
  //     </div>
  //   );
  // }

  return (
    <React.Fragment>
      <Form {...layout} onFinish={handleApply} form={form}>
        <Row gutter={[0, 24]}>
          <Col span="24">
            <Form.Item label="Internet IP">
              <Input
                disabled
                size="large"
                placeholder="0.0.0.0"
                className="label-info"
                value={internet_ip}
              />
            </Form.Item>
          </Col>
          <Col span={24}>
            <Form.Item label="Subnet Mask">
              <Input
                disabled
                size="large"
                placeholder="0.0.0.0"
                className="label-info"
                value={netmask}
              />
            </Form.Item>
          </Col>
          <Col span={24}>
            <Form.Item label="Default Getway">
              <Input
                disabled
                size="large"
                placeholder="0.0.0.0"
                className="label-info"
                value={gateway}
              />
            </Form.Item>
          </Col>
          <Col span={24}>
            <Form.Item label="DNS">
              <Input
                disabled
                size="large"
                placeholder="0.0.0.0"
                className="label-info"
                value={dns}
              />
            </Form.Item>
          </Col>
        </Row>
        <Form.Item>
          <Button
            type="primary"
            htmlType="submit"
            className="button-apply"
            size="large"
          >
            SAVE & APPLY
          </Button>
        </Form.Item>
      </Form>
    </React.Fragment>
  );
};

export default WANDynamic;
