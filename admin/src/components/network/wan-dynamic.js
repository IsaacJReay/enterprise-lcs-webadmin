import React, { useEffect, useState } from "react";
import { Form, Input, Button, message, Spin } from "antd";
import axios from "axios";

const getToken = localStorage.getItem("token");

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
        url: "http://10.42.0.188:8080/private/api/settings/wirednetwork/status",
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
      url: "http://10.42.0.188:8080/private/api/settings/wirednetwork/dynamic",
      headers: {
        "content-type": "application/json",
        ...auth,
      },
    })
      .then(async (res) => {
        if (res.data.operation_status === "Success") {
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
        <Form.Item label="Internet IP">
          <Input
            disabled
            size="large"
            placeholder="0.0.0.0"
            className="label-info"
            value={items.internet_ip}
          />
        </Form.Item>
        <Form.Item label="Subnet Mask">
          <Input
            disabled
            size="large"
            placeholder="0.0.0.0"
            className="label-info"
            value={items.netmask}
          />
        </Form.Item>
        <Form.Item label="Default Getway">
          <Input
            disabled
            size="large"
            placeholder="0.0.0.0"
            className="label-info"
            value={items.gateway}
          />
        </Form.Item>
        <Form.Item label="DNS">
          <Input
            disabled
            size="large"
            placeholder="0.0.0.0"
            className="label-info"
            value={items.dns}
          />
        </Form.Item>
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
      </Form>
    </React.Fragment>
  );
};

export default WANDynamic;
