import React, { useEffect, useState } from "react";
import { Form, Input, Button, message, Spin, Row, Col } from "antd";
import axios from "axios";

const getToken = localStorage.getItem("token");
const baseUrl = process.env.REACT_APP_API_URL;

const WANDynamic = () => {
  const [items, setItems] = useState({});
  const [loading, setLoading] = useState(false);

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

  useEffect(() => {
    async function fetchData() {
      await axios({
        method: "GET",
        url: `${baseUrl}/settings/wirednetwork/status`,
        headers: {
          "content-type": "application/json",
          ...auth,
        },
      })
        .then((res) => {
          setItems(res.data.wired_network_param);
          setTimeout(() => {
            setLoading(false);
          }, 1000);
        })
        .catch((err) => console.log(err));
    }
    fetchData();
  }, []);

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
          setTimeout(() => {
            message.success("Successful!");
          }, 1000);
        } else {
          setLoading(true);
          message.error("operation failed! ");
          setLoading(false);
        }
      })

      .catch((err) => {
        setTimeout(() => {
          setLoading(false);
        }, 1000);
        message.error(err.response.data.reason);
      });
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
      <Form {...layout} onFinish={handleApply}>
        <Row gutter={[0, 24]}>
          <Col span="24">
            <Form.Item label="Internet IP">
              <Input
                disabled
                size="large"
                placeholder="0.0.0.0"
                className="label-info"
                value={items.internet_ip}
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
                value={items.netmask}
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
                value={items.gateway}
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
                value={items.dns}
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
